use std::io;
use std::io::{BufReader, Stdin};
use std::io::prelude::*;
use std::iter::Iterator;
use std::path::{Path, PathBuf};

use clap::Parser;


fn main()
{
    let args = CliArgs::parse();

    let stdin_reader = BufReader::new(std::io::stdin());

    do_it(resolve_renamer(args.x), stdin_reader);
}

/// Read file names from stdin (one per line) and rename the file so that its extension is lowercase
#[derive(Parser)]
struct CliArgs
{
    /// Do execute the action (default is to run dry)
    #[arg(short)]
    x: bool,
}

fn do_it(mut renamer: Renamer, file_names_reader: BufReader<Stdin>)
{
    let paths = file_names_reader.lines().filter_map(try_get_path);
    let rename_or_skip_list = paths.map(analyze_path_string);
    for rename_or_skip in rename_or_skip_list {
        match rename_or_skip {
            Ok(r) => renamer.apply(&r),
            Err(skip) => report_skip(&skip),
        }
    }
}

struct Rename
{
    from: PathBuf,
    to: PathBuf,
}

struct RenameReporter
{
    stream: Box<dyn Write>,
}

impl RenameReporter
{
    fn report(&mut self, rename: &Rename)
    {
        let msg = format!("{} -> {}\n",
                               rename.from.display(),
                               rename.to.display());
        write_formatted(&mut self.stream, msg);
    }
}

fn write_formatted(stream: &mut Box<dyn Write>, msg: String)
{
    match stream.write(msg.into_bytes().as_slice()) {
        Ok(_) => (),
        Err(io_error) => eprintln!("IO error while writing: {}", io_error),
    }
}
fn report_skip(skip: &Skip)
{
    eprintln!("{}: {}", skip.path.display(), skip.msg);
}

struct Renamer
{
    reporter: RenameReporter,
    action: fn(&Rename) -> Option<io::Error>,
    io_error_reporter: fn(&PathBuf, error: &io::Error),
}

enum DoNotRenameCause
{
    ASkip(Skip),
    IoError(io::Error),
}
impl Renamer
{
    fn apply(&mut self, rename: &Rename)
    {
        match self.should_try_rename(&rename) {
            Some(do_not_rename_cause) => {
                match do_not_rename_cause {
                    DoNotRenameCause::ASkip(skip) => report_skip(&skip),
                    DoNotRenameCause::IoError(io_error) => (self.io_error_reporter)(&rename.from, &io_error)
                }
            }
            None => {
                match (self.action)(rename) {
                    None => self.reporter.report(rename),
                    Some(io_error) => (self.io_error_reporter)(&rename.from, &io_error)
                }
            }
        }
    }

    fn should_try_rename(&self, rename: &Rename) -> Option<DoNotRenameCause>
    {
        let meta_data_result = rename.from.metadata();
        match meta_data_result {
            Err(io_error) => Some(DoNotRenameCause::IoError(io_error)),
            Ok(meta_data) => {
                if meta_data.is_file() { 
                    self.check_dst_exists(rename)
                }
                else {
                    Some(DoNotRenameCause::ASkip(Skip::new(String::from("not a regular file"), rename.from.clone()))) 
                }
            }
        }
    }

    fn check_dst_exists(&self, rename: &Rename) -> Option<DoNotRenameCause>
    {
        let exists_r = rename.to.try_exists();
        match exists_r {
            Err(io_error) => {
                let msg = format!("cannot check DST for existence: {}", io_error.to_string());
                Some(DoNotRenameCause::ASkip(
                Skip::new(
                    msg, 
                rename.from.clone()))
            ) },
            Ok(dst_exists) => {
                if dst_exists { 
                    Some(DoNotRenameCause::ASkip(Skip::new(String::from("DST exists"), rename.from.clone()))) 

                }
                else {
                    None
                }
            }
        }
    }
}

fn resolve_renamer(execute: bool) -> Renamer
{
    if execute {
        Renamer {
            reporter: RenameReporter { stream: Box::new(io::stdout()) },
            action: do_rename,
            io_error_reporter: report_io_error,
        }
    } else {
        Renamer {
            reporter: RenameReporter { stream: Box::new(io::stderr()) },
            action: do_nothing,
            io_error_reporter: report_io_error,
        }
    }
}

fn do_nothing(_rename: &Rename) -> Option<io::Error>
{
    None
}

fn do_rename(rename: &Rename) -> Option<io::Error>
{
    match std::fs::rename(rename.from.as_path(), rename.to.as_path()) {
        Ok(_) => None,
        Err(io_error) => Some(io_error),
    }
}

struct Skip
{
    msg: String,
    path: PathBuf,
}

impl Skip
{
    fn new(msg: String, path: PathBuf) -> Self
    {
        Skip { msg, path }
    }
    fn new_err<T>(msg: String, path: PathBuf) -> Result<T, Self>
    {
        Err(Skip::new(msg, path))
    }

}

fn analyze_path_string(path: PathBuf) -> Result<Rename, Skip>
{
    let file_name = path.file_name()
        .ok_or_else(|| Skip::new(String::from("path without file name"), path.clone()))
        .map(Path::new)?;

    let ext_os_str = file_name.extension()
        .ok_or_else(|| Skip::new(String::from("path without extension"), path.clone()))?;

    let ext_str = ext_os_str.to_str()
        .ok_or_else(|| Skip::new(String::from("path with non unicode extension"), path.clone()))?;

    let ext_str_lower = String::from(ext_str).to_lowercase();
    if ext_str_lower.eq(ext_str) {
        Skip::new_err(String::from("path with lowercase extension"), path.clone())
    } else {
        let path_w_lowercase_ext = path.with_extension(ext_str_lower);
        Ok(Rename { from: path, to: path_w_lowercase_ext })
    }
}

fn try_get_path(item: std::io::Result<String>) -> Option<PathBuf>
{
    match item {
        Ok(s) => Some(PathBuf::from(s)),
        Err(e) => report_invalid_path(&e.to_string()),
    }
}

fn report_invalid_path<T>(msg: &str) -> Option<T>
{
    eprintln!("{}", msg);
    None
}

fn report_io_error(path: &PathBuf, error: &io::Error)
{
    eprintln!("{:?}: {}", path.to_str(), error);
}
