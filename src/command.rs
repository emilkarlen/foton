use std::io;
use crate::command::CmdError::ExecutionError;

pub enum CmdError
{
    ArgsValidationError(String),
    ExecutionError(ExeError),
}

pub enum ExeError
{
    UnableToExecute(UnableToExecuteError),
    OsError(io::Error),
}

pub struct UnableToExecuteError
{
    pub reason: String,
    pub details: Vec<String>
}

pub trait ExecutableCmd {
    fn execute(&self) -> Result<(), CmdError>;
}

pub fn io_to_cmd(e: io::Error) -> CmdError
{
    ExecutionError(ExeError::OsError(e))
}

pub fn io_to_exe(e: io::Error) -> ExeError
{
    ExeError::OsError(e)
}

pub fn exe_to_cmd(e: ExeError) -> CmdError
{
    CmdError::ExecutionError(e)
}
