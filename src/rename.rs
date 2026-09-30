pub mod cli;
mod read_files;
mod rename_files;
mod report;
mod common;
mod naming;
mod config;
pub mod main;
pub mod custom_format;
mod renamer;

use crate::command;
use crate::command::{CmdError, ExeError, ExecutableCmd};
use crate::common::arg_validation;
use crate::rename::config::{NamingConfig, NamingConfigCli};
use crate::rename::custom_format::FormatPart;
use config::ReadConfig;
use std::path::PathBuf;

pub struct CmdConfig
{
    pub execute: bool,
    pub directory: PathBuf,
    pub read_config: ReadConfig,
    pub naming_config: NamingConfigCli,
}

impl ExecutableCmd for CmdConfig
{
    fn execute(&self) -> Result<(), CmdError>
    {
        let naming_config = self.validate_args().map_err(CmdError::ArgsValidationError)?;
        self.with_valid_args(&naming_config).map_err(command::exe_to_cmd)
    }
}

impl CmdConfig
{
    fn validate_args(&self) -> Result<NamingConfig, String>
    {
        arg_validation::is_existing_dir(self.directory.as_ref())?;
        match &self.naming_config.format {
            None => self.nc_of(None),
            Some(format_str) => {
                let format_parts = custom_format::parse(format_str)?;
                self.nc_of(Some(format_parts))
            }
        }
    }

    fn with_valid_args(&self, naming_config: &NamingConfig) -> Result<(), ExeError>
    {
        main::main(&self.directory, self.execute, &self.read_config, naming_config)
    }

    fn nc_of(&self, format: Option<Vec<FormatPart>>) -> Result<NamingConfig, String> {
        Ok(NamingConfig {
            start_num: self.naming_config.start_num,
            min_width: self.naming_config.min_width,
            format: format,
        })

    }
}
