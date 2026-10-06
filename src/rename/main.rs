use super::config::NamingConfig;
use super::{naming, report};
use crate::command::{io_to_exe, ExeError};
use crate::rename::read_files::sorted_file_infos;
use std::path::PathBuf;

pub fn main(dirs: Vec<PathBuf>, execute: bool, read_config: &crate::common::read_files::ReadConfig, naming_config: &NamingConfig) ->  Result<(), ExeError>
{
    let files = sorted_file_infos(dirs, read_config).map_err(io_to_exe)?;
    let renames = naming::resolve(files, naming_config)?;
    if execute {
        crate::rename::rename_files::execute(&renames).map_err(io_to_exe)?;
    }
    else {
        report::report_renames(&renames);
    }
    Ok(())
}
