pub mod cli;
mod err;
mod err_fmt;
mod renamer;
mod rename;
mod reporter;

use std::io;
use crate::command;
use self::renamer::Renamer;
use self::reporter::Reporter;
use super::command::{CmdError, ExecutableCmd};

pub struct CmdConfig
{
    pub execute: bool,
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
        Ok(())
    }

    fn with_valid_args(&self) -> io::Result<()>
    {
        let mut reporter = Reporter::new();
        let renamer = Renamer::resolve(self.execute);

        rename::rename_files(std::io::stdin(), renamer,  &mut reporter)?;

        Ok(())
    }
}
