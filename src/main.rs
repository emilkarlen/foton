use std::io;
use std::io::Stdin;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use std::convert::From;

use clap::Parser;


fn main() -> std::process::ExitCode
{
    let args = CliArgs::parse();

    let mut reporter = Reporter::new();
    let renamer = Renamer::resolve(args.x);

    do_it(std::io::stdin(), renamer,  &mut reporter)
}

/// Read file names from stdin (one per line) and rename the file so that its extension is lowercase
#[derive(clap::Parser)]
struct CliArgs
{
    /// Do execute the action (default is to run dry)
    #[arg(short)]
    x: bool,
}

fn do_it(file_names_reader: Stdin, renamer: Renamer,reporter: &mut Reporter) -> std::process::ExitCode
{
    let mut exit_code = std::process::ExitCode::SUCCESS;

    for input_line_r in file_names_reader.lines() {
        match input_line_r {
            Err(io_err) => {
                reporter.report_stdin_read_error(&io_err);
                exit_code = std::process::ExitCode::FAILURE;
                break;
            }
            Ok(input_line) => {
                process_file_name(&renamer, reporter, &input_line);
            }
        }
    }
    exit_code
}

fn process_file_name(renamer: &Renamer, reporter: &mut Reporter, file_name: &String)
{
    let src = PathBuf::from(file_name);
    match analyze_path_string(&src) {
        Err(err) => {
            reporter.report_src_str_err(file_name, &err);
        }
        Ok(dst) => {
            let rename = Rename { src, dst };
            match renamer.apply(&rename) {
                Err(err) => {
                    reporter.report_fs_error(file_name, &rename, &err);
                }
                Ok(_) => {
                    reporter.report_rename(&rename);
                }
            }
        }
    }
}

/// Invalid path string
enum PathStrError
{
    MissingFileName,
    MissingExtension,
    NonUnicodeExtension,
    LowercaseExtension,
}

/// Invalid file system files
enum PathFsError
{
    Src(SrcPathFsError),
    Dst(DstPathFsError),
    Rename(io::Error),
}

enum SrcPathFsError
{
    Access(io::Error),
    NotARegularFile,
}

enum DstPathFsError
{
    ExistenceCheck(io::Error),
    Exists,
}

struct Rename
{
    src: PathBuf,
    dst: PathBuf,
}
struct Reporter
{
    rename: Box<dyn Write>,
    skip: Box<dyn Write>,
    io_error: Box<dyn Write>,
}

impl Reporter
{
    fn new() -> Self
    {
        Reporter
        {
            rename: Box::new(io::stdout()),
            skip: Box::new(io::stderr()),
            io_error: Box::new(io::stderr()),
        }
    }

    fn report_src_str_err(&mut self, file_name: &String, cause: &PathStrError)
    {
        let explanation = err_fmt::src_str_err(cause);
        self.report_skip(file_name, &explanation);
    }

    fn report_fs_error(&mut self, file_name: &String, rename: &Rename, cause: &PathFsError)
    {
        let explanation = err_fmt::fs_err(cause, &rename.dst);
        self.report_skip(file_name, &explanation);
    }

    fn report_rename(&mut self, rename: &Rename)
    {
        let msg = format!("{} -> {}\n",
                               rename.src.to_string_lossy(),
                               rename.dst.to_string_lossy());
        Reporter::write_to_stream(&mut self.rename,  msg)
    }

    fn report_skip(&mut self, file_name: &String, cause: &str)
    {
        let msg = err_fmt::format_skip(file_name, cause);
        Reporter::write_to_stream(&mut self.skip,  msg)
    }

    fn report_stdin_read_error(&mut self, error: &io::Error)
    {
        let msg = format!("{}\n", error.to_string());
        Reporter::write_to_stream(&mut self.io_error,  msg)
    }

    fn write_to_stream(stream: &mut Box<dyn Write>, msg: String)
    {
        match stream.write(msg.into_bytes().as_slice()) {
            Ok(_) => (),
            Err(io_error) => eprintln!("IO error while writing: {}", io_error),
        }
    }
}

mod err_fmt
{
    use crate::PathStrError;
    use crate::PathFsError;
    use crate::SrcPathFsError;
    use crate::DstPathFsError;
    use std::path::PathBuf;
    use std::io;
    
    pub fn format_skip(file_name: &String, cause: &str) -> String
    {
        format!("{}: {}\n", file_name, cause)
    }

    pub fn fs_err(cause: &PathFsError, dst: &PathBuf) -> String
    {
        match cause {
            PathFsError::Src(err) => fs_src_err(err),
            PathFsError::Dst(err) => fs_dst_err(err, dst),
            PathFsError::Rename(io_err) => fs_rename_err(io_err, dst),
        }
    }

    pub fn src_str_err(cause: &PathStrError) -> &'static str
    {
        match cause {
            PathStrError::MissingFileName =>
                "missing file name",
            PathStrError::MissingExtension =>
                "missing extension",
            PathStrError::NonUnicodeExtension =>
                "non-Unicode extension",
            PathStrError::LowercaseExtension =>
                "extension is lowercase",
        }
    }

    fn fs_src_err(cause: &SrcPathFsError) -> String
    {
        match cause {
            SrcPathFsError::Access(io_error) => io_error.to_string(),
            SrcPathFsError::NotARegularFile => String::from("not a regular file"),
        }
    }
    
    fn fs_dst_err(cause: &DstPathFsError, dst: &PathBuf) -> String
    {
        match cause {
            DstPathFsError::ExistenceCheck(io_error) => io_error.to_string(),
            DstPathFsError::Exists => format!("Exists: {}", dst.to_string_lossy()),
        }
    }
        
    fn fs_rename_err(cause: &io::Error, _dst: &PathBuf) -> String
    {
        cause.to_string()
    }
}

struct Renamer
{
    action: fn(&Rename) -> Result<(), io::Error>,
}

impl Renamer
{

    fn resolve(execute: bool) -> Renamer
    {
        if execute {
            Renamer {
                action: Renamer::do_rename,
            }
        } else {
            Renamer {
                action: Renamer::do_nothing,
            }
        }
    }

    fn apply(&self, rename: &Rename) -> Result<(), PathFsError>
    {
        self.check(rename)?;
        (self.action)(rename).map_err(|e| PathFsError::Rename(e))
    }

    fn check(&self, rename: &Rename) -> Result<(), PathFsError>
    {
        self.check_src(&rename.src).map_err(|se| PathFsError::Src(se))?;
        self.check_dst(&rename.dst).map_err(|se| PathFsError::Dst(se))
    }

    fn check_src(&self, src: &PathBuf) -> Result<(), SrcPathFsError>
    {
        let meta_data_result = src.metadata();
        match meta_data_result {
            Err(io_error) => Err(SrcPathFsError::Access(io_error)),
            Ok(meta_data) => {
                if meta_data.is_file() { 
                    Ok(())
                }
                else {
                    Err(SrcPathFsError::NotARegularFile) 
                }
            }
        }
    }

    fn check_dst(&self, dst: &PathBuf) -> Result<(), DstPathFsError>
    {
        let exists_r = dst.try_exists();
        match exists_r {
            Err(io_error) => {
                Err(DstPathFsError::ExistenceCheck(io_error)
            ) },
            Ok(dst_exists) => {
                if dst_exists { 
                    Err(DstPathFsError::Exists) 
                }
                else {
                    Ok(())
                }
            }
        }
    }

    fn do_nothing(_rename: &Rename) -> Result<(), io::Error>
    {
        Ok(())
    }

    fn do_rename(rename: &Rename) -> Result<(), io::Error>
    {
        std::fs::rename(rename.src.as_path(), rename.dst.as_path())
    }

}

fn analyze_path_string(path: &PathBuf) -> Result<PathBuf, PathStrError>
{
    let file_name = path.file_name()
        .ok_or_else(|| PathStrError::MissingFileName)
        .map(Path::new)?;

    let ext_os_str = file_name.extension()
        .ok_or_else(|| PathStrError::MissingExtension)?;

    let ext_str = ext_os_str.to_str()
        .ok_or_else(|| PathStrError::NonUnicodeExtension)?;

    let ext_str_lower = String::from(ext_str).to_lowercase();
    if ext_str_lower.eq(ext_str) {
        Err(PathStrError::LowercaseExtension)
    } else {
        let path_w_lowercase_ext = path.with_extension(ext_str_lower);
        Ok(path_w_lowercase_ext)
    }
}
