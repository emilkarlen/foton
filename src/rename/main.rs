use super::config::NamingConfig;
use super::{naming, read_files, report};
use crate::command::{io_to_exe, ExeError};
use crate::common::read_files::{PathWithName, ReadConfig};

pub fn main(dirs: Vec<PathWithName>, execute: bool, read_config: &ReadConfig, naming_config: &NamingConfig) ->  Result<(), ExeError>
{
    let files = read_files::rev_sorted_file_infos(dirs, read_config).map_err(io_to_exe)?;
    let renames = naming::resolve(files, naming_config)?;
    // dbg!(&renames);
    if execute {
        crate::rename::rename_files::execute(&renames).map_err(io_to_exe)?;
    }
    else {
        report::report_renames(&renames);
    }
    Ok(())
}
