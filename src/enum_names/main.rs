use crate::common::dir_contents::DirContents;
use super::common::Rename;
use super::config::{NamingConfig, ReadConfig};
use super::{naming, read_files, report};
use std::io;
use std::path::PathBuf;

pub fn main(dir: &PathBuf, execute: bool, read_config: &ReadConfig, naming_config: &NamingConfig) ->  io::Result<()>
{
    let renames = get_renames(dir.clone(), read_config, naming_config)?;
    if execute {
        crate::enum_names::rename_files::execute(&renames)?;
    }
    else {
        report::report_renames(&renames);
    }
    Ok(())
}

fn get_renames(dir: PathBuf, read_config: &ReadConfig, naming_config: &NamingConfig) -> io::Result<DirContents<Vec<Rename>>>
{
    let mut files = read_files::rev_sorted_file_infos(dir, read_config)?;
    Ok(naming::resolve(&mut files, naming_config))
}
