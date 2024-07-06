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

    let reporter = Reporter::new();

    do_it(resolve_renamer(args.x), stdin_reader, &reporter);
}

/// Read file names from stdin (one per line) and rename the file so that its extension is lowercase
#[derive(Parser)]
struct CliArgs
{
    /// Do execute the action (default is to run dry)
    #[arg(short)]
    x: bool,
}

fn do_it(renamer: Renamer, file_names_reader: BufReader<Stdin>, reporter: &Reporter)
{
    // let try_get_path2 = |path: std::io::Result<String>| -> Option<PathBuf>  {
    //     match path {
    //         Ok(s) => Some(PathBuf::from(s)),
    //         Err(e) => {reporter.report_invalid_path(&e); None },
    //     }
    // };

    // let paths = file_names_reader.lines().filter_map(try_get_path2);

    let paths = get_paths(file_names_reader, reporter);
    let rename_or_skip_list = paths.map(analyze_path_string);
    for rename_or_skip in rename_or_skip_list {
        match rename_or_skip {
            Ok(r) => renamer.apply(&r, reporter),
            Err(skip) => reporter.report_skip(&skip),
        }
    }
}

fn get_paths(file_names_reader: BufReader<Stdin>, reporter: &Reporter) -> impl Iterator<Item = PathBuf> + '_
{
    let try_get_path2 = |path: std::io::Result<String>| -> Option<PathBuf>  {
        match path {
            Ok(s) => Some(PathBuf::from(s)),
            Err(e) => {reporter.report_invalid_path(&e); None },
        }
    };

    file_names_reader.lines().filter_map(try_get_path2)
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

enum DoNotRenameCause
{
    ASkip(Skip),
    IoError(io::Error),
}

struct Rename
{
    from: PathBuf,
    to: PathBuf,
}

type WriteGetter = fn() -> Box<dyn Write>;

struct Reporter
{
    rename: WriteGetter,
    skip: WriteGetter,
    io_error: WriteGetter,

}

impl Reporter
{
    fn new() -> Self
    {
        Reporter
        {
            rename: || Box::new(io::stdout()),
            skip: || Box::new(io::stderr()),
            io_error: || Box::new(io::stderr()),
        }
    }

    fn report_rename(&self, rename: &Rename)
    {
        let msg = format!("{} -> {}\n",
                               rename.from.display(),
                               rename.to.display());
        write_to_stream(self.rename,  msg)
    }

    fn report_skip(&self, skip: &Skip)
    {
        let msg = format!("{}: {}\n",
                               skip.path.display(),
                               skip.msg);
        write_to_stream(self.skip,  msg)
    }


    fn report_io_error(&self, path: &PathBuf, error: &io::Error)
    {
        let msg = format!("{}: {}\n",
                               path.display(),
                               error.to_string());
        write_to_stream(self.io_error,  msg)
    }

    fn report_invalid_path(&self, error: &io::Error)
    {
        let msg = format!("{}\n",
                               error.to_string());
        write_to_stream(self.io_error,  msg)
    }
}

fn write_to_stream(stream: WriteGetter, msg: String)
{
    match stream().write(msg.into_bytes().as_slice()) {
        Ok(_) => (),
        Err(io_error) => eprintln!("IO error while writing: {}", io_error),
    }
}

struct Renamer
{
    action: fn(&Rename) -> Option<io::Error>,
}

impl Renamer
{
    fn apply(&self, rename: &Rename, reporter: &Reporter)
    {
        match self.should_try_rename(&rename) {
            Some(do_not_rename_cause) => {
                match do_not_rename_cause {
                    DoNotRenameCause::ASkip(skip) => reporter.report_skip(&skip),
                    DoNotRenameCause::IoError(io_error) => reporter.report_io_error(&rename.from, &io_error)
                }
            }
            None => {
                match (self.action)(rename) {
                    None => reporter.report_rename(rename),
                    Some(io_error) => reporter.report_io_error(&rename.from, &io_error)
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
            action: do_rename,
        }
    } else {
        Renamer {
            action: do_nothing,
        }
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
