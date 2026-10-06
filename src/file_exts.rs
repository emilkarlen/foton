pub mod cli;
mod report;
mod config;
pub mod files_builder;
mod read_files;

use crate::command;
use crate::command::{CmdError, ExecutableCmd};
use crate::common::arg_validation;
use crate::common::read_files::ReadConfig;
use crate::file_exts::config::ReportConfig;
use std::io;
use std::path::PathBuf;

pub struct CmdConfig
{
    pub read_config: ReadConfig,
    pub report_config: ReportConfig,
    pub directories: Vec<PathBuf>,
}

impl ExecutableCmd for CmdConfig
{
    fn execute(&self) -> Result<(), CmdError>
    {
        self.validate_args().map_err(CmdError::ArgsValidationError)?;
        self.with_valid_args().map_err(command::io_to_cmd)
    }
}

impl CmdConfig
{
    fn validate_args(&self) -> Result<(), String>
    {
        for dir in self.directories.iter() {
            arg_validation::is_existing_dir(dir.as_ref())?;
        }
        Ok(())
    }
    fn with_valid_args(&self) -> io::Result<()>
    {
        let mut extensions = read_files::execute(self.directories.clone(), &self.read_config)?;
        report::execute(&self.report_config, &mut extensions);
        Ok(())
    }
}
