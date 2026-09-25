pub mod cli;
mod read_files;
mod stem_formatter;
mod rename_files;
mod report;
mod ext_filter;
mod common;
mod naming;

use crate::command::ExecutableCmd;
use crate::common::ext_filter::ExtensionsFilter;
use crate::enum_names::common::DirContents;
use crate::enum_names::rename_files::execute;
use common::Rename;
use std::io;
use std::io::Write;
use std::ops::Deref;
use std::path::Path;
use std::process::ExitCode;
use crate::common::cli_exit::EXIT_INVALID_ARG;

pub struct CmdConfig
{
    pub execute: bool,
    pub recursive: bool,
    pub directory: Box<Path>,
    pub extensions_filter: Box<dyn ExtensionsFilter>,
}

impl ExecutableCmd for CmdConfig
{
    fn execute(&self) -> ExitCode
    {
        if !self.directory.is_dir() {
            std::io::stderr().write_fmt(format_args!("not a dir: {}\n", self.directory.display())).unwrap();
            return ExitCode::from(EXIT_INVALID_ARG);
        }
        match self.with_valid_args() {
            Ok(exit_code) => ExitCode::from(exit_code),
            Err(e) => {
                io::stderr().write_fmt(format_args!("{}\n", e)).expect("Could not write to stderr");
                ExitCode::FAILURE
            }
        }
    }
}

impl CmdConfig
{
    fn with_valid_args(&self) -> io::Result<u8>
    {
        let renames = get_renames(&self.directory, self.recursive, self.extensions_filter.deref())?;
        if self.execute {
            execute(&renames)?;
        }
        else {
            report_renames(&renames);
        }
        Ok(0)

    }
}

fn report_renames(rid: &DirContents<Rename>)
{
    for rename in rid.files.iter() {
        for (old_fn, new_fn) in rename.renames().iter() {
            report::report_rename(&rid.dir, (old_fn, new_fn));
        }
    }
    for sub_dir in rid.sub_dirs.iter() {
        report_renames(&sub_dir);
    }
}

fn get_renames(dir: &Path, recursive: bool, extensions_filter: &dyn ExtensionsFilter) -> io::Result<DirContents<Rename>>
{
    let mut files = read_files::rev_sorted_file_infos(dir, recursive, extensions_filter)?;
    Ok(naming::resolve(&mut files))
}
