pub mod cli;
pub mod main;
pub mod config;

use crate::command;
use crate::command::{CmdError, ExecutableCmd};
use crate::common::arg_validation;
use crate::common::read_files::ReadConfig;
use crate::sync_deletions::config::ProcessConfig;
use std::io;
use std::path::PathBuf;

pub struct CmdConfig
{
    pub process_config: ProcessConfig,
    pub dir_src: PathBuf,
    pub dir_dst: PathBuf,
    pub read_config: ReadConfig,
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
        let mt = &(&self.process_config).move_to;
        if  let Some(dir_move_to) = mt {
            arg_validation::is_existing_dir(dir_move_to.as_ref())?;
        }
        arg_validation::is_existing_dir(self.dir_src.as_ref())?;
        arg_validation::is_existing_dir(self.dir_dst.as_ref())?;

        Ok(())
    }
    fn with_valid_args(&self) -> io::Result<()>
    {
        main::with_valid_args(&self.process_config, &self.dir_src, &self.dir_dst, &self.read_config)
    }
}
