pub mod cli;
mod read_files;
mod stem_formatter;
mod rename_files;
mod report;
mod common;
mod naming;
mod config;
pub mod main;

use crate::command::ExecutableCmd;
use crate::common::arg_validation;
use crate::enum_names::config::NamingConfig;
use config::ReadConfig;
use std::io;
use std::path::PathBuf;

pub struct CmdConfig
{
    pub execute: bool,
    pub directory: PathBuf,
    pub read_config: ReadConfig,
    pub naming_config: NamingConfig,
}

impl ExecutableCmd for CmdConfig
{
    fn validate_args(&self) -> Result<(), String>
    {
        arg_validation::is_existing_dir(self.directory.as_ref())
    }

    fn with_valid_args(&self) -> io::Result<()>
    {
        main::main(&self.directory, self.execute, &self.read_config, &self.naming_config)
    }
}
