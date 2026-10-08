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
pub mod cli_validation;

use crate::command;
use crate::command::{CmdError, ExeError, ExecutableCmd};
use crate::rename::config::{NamingConfig, NamingConfigCli};
use std::path::PathBuf;

pub struct CmdConfig
{
    pub execute: bool,
    pub directories: Vec<PathBuf>,
    pub read_config: crate::common::read_files::config::ReadConfig,
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
        cli_validation::validate_args(&self.directories, &self.naming_config)
    }

    fn with_valid_args(&self, naming_config: &NamingConfig) -> Result<(), ExeError>
    {
        main::main(self.directories.clone(), self.execute, &self.read_config, naming_config)
    }
}
