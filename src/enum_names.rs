pub mod cli;
mod read_files;
mod stem_formatter;
mod rename_files;
mod report;
mod common;
mod naming;
mod config;

use crate::command::ExecutableCmd;
use crate::common::arg_validation;
use crate::common::dir_contents::DirContents;
use crate::enum_names::rename_files::execute;
use common::Rename;
use config::ReadConfig;
use std::io;
use std::path::PathBuf;

pub struct CmdConfig
{
    pub execute: bool,
    pub directory: PathBuf,
    pub read_config: ReadConfig,
}

impl ExecutableCmd for CmdConfig
{
    fn validate_args(&self) -> Result<(), String>
    {
        arg_validation::is_existing_dir(self.directory.as_ref())
    }

    fn with_valid_args(&self) -> io::Result<()>
    {
        let renames = get_renames(self.directory.clone(), &self.read_config)?;
        if self.execute {
            execute(&renames)?;
        }
        else {
            report_renames(&renames);
        }
        Ok(())
    }
}

fn report_renames(rid: &DirContents<Vec<Rename>>)
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

fn get_renames(dir: PathBuf, config: &ReadConfig) -> io::Result<DirContents<Vec<Rename>>>
{
    let mut files = read_files::rev_sorted_file_infos(dir, config)?;
    Ok(naming::resolve(&mut files))
}
