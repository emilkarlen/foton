use super::config::{NamingConfig, ReadConfig};
use super::{naming, read_files, report};
use crate::command::{io_to_exe, ExeError};
use std::path::PathBuf;

pub fn main(dir: &PathBuf, execute: bool, read_config: &ReadConfig, naming_config: &NamingConfig) ->  Result<(), ExeError>
{
    let files = read_files::rev_sorted_file_infos(dir.clone(), read_config).map_err(io_to_exe)?;
    let renames = naming::resolve(files, naming_config)?;
    if execute {
        crate::rename::rename_files::execute(&renames).map_err(io_to_exe)?;
    }
    else {
        report::report_renames(&renames);
    }
    Ok(())
}

