pub mod cli;
mod read_files;
mod report;
mod config;

use std::io;
use std::path::Path;
use crate::command::ExecutableCmd;
use crate::common::arg_validation;
use crate::file_exts::config::{ReportConfig, ReadConfig};

pub struct CmdConfig
{
    pub read_config: ReadConfig,
    pub report_config: ReportConfig,
    pub directories: Vec<Box<Path>>,
}

impl ExecutableCmd for CmdConfig
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
        let mut extensions = read_files::execute(&self.directories, &self.read_config)?;
        report::execute(&self.report_config, &mut extensions);
        Ok(())
    }
}
