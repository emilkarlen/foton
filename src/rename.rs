pub mod cli;
mod rename_files;
mod report;
mod common;
mod config;
pub mod custom_format;
mod renamer;
mod read_files;
mod main;
mod naming;

use crate::command;
use crate::command::{CmdError, ExeError, ExecutableCmd};
use crate::common::arg_validation;
use crate::rename::config::{NamingConfig, NamingConfigCli};
use crate::rename::custom_format::FormatPart;
use std::path::PathBuf;

pub struct CmdConfig
{
    pub execute: bool,
    pub directory: PathBuf,
    pub read_config: read_files::ReadConfig,
    pub naming_config: NamingConfigCli,
}

impl ExecutableCmd for CmdConfig
{
    fn execute(&self) -> Result<(), CmdError>
    {
        let (dir, naming_config) = self.validate_args().map_err(CmdError::ArgsValidationError)?;
        self.with_valid_args(dir, &naming_config).map_err(command::exe_to_cmd)
    }
}

impl CmdConfig
{
    fn validate_args(&self) -> Result<(crate::common::read_files::PathWithName, NamingConfig), String>
    {
        arg_validation::is_existing_dir(self.directory.as_ref())?;
        let naming_config = self.resolve_naming_args()?;
        let pwn = arg_validation::is_path_with_name(&self.directory, "DIR")?;
        Ok((pwn, naming_config))
    }

    fn resolve_naming_args(&self) -> Result<NamingConfig, String>
    {
        match &self.naming_config.format {
            None => self.nc_of(None),
            Some(format_str) => {
                let format_parts = custom_format::parse(format_str)?;
                self.nc_of(Some(format_parts))
            }
        }
    }

    fn with_valid_args(&self, dir: read_files::PathWithName, naming_config: &NamingConfig) -> Result<(), ExeError>
    {
        main::main(dir, self.execute, &self.read_config, naming_config)
    }

    fn nc_of(&self, format: Option<Vec<FormatPart>>) -> Result<NamingConfig, String> {
        Ok(self.nc_of_plain(format))
    }

    fn nc_of_plain(&self, format: Option<Vec<FormatPart>>) -> NamingConfig {
        NamingConfig {
            start_num: self.naming_config.start_num,
            min_width: self.naming_config.min_width,
            format: format,
        }    }
}
