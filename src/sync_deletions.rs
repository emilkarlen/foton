pub mod cli;

use crate::command::ExecutableCmd;
use crate::common::cli_exit::EXIT_INVALID_ARG;
use crate::common::arg_validation;
use crate::common::ext_filter::ExtensionsFilter;
use std::io;
use std::path::Path;
use std::process::ExitCode;

pub struct CmdConfig
{
    pub execute: bool,
    pub recursive: bool,
    pub dir_src: Box<Path>,
    pub dir_dst: Box<Path>,
    pub extensions_filter: Box<dyn ExtensionsFilter>,
}

impl ExecutableCmd for CmdConfig
{
    fn execute(&self) -> ExitCode
    {
        if let Err(err_msg) = self.validate_args() {
            eprintln!("{}", err_msg);
            return ExitCode::from(EXIT_INVALID_ARG);
        }
        match self.with_valid_args() {
            Ok(exit_code) => ExitCode::from(exit_code),
            Err(e) => {
                eprintln!("{}\n", e);
                ExitCode::FAILURE
            }
        }
    }
}


impl CmdConfig
{
    fn validate_args(&self) -> Result<(), String>
    {
        arg_validation::is_existing_dir(self.dir_src.as_ref())?;
        arg_validation::is_existing_dir(self.dir_src.as_ref())?;
        arg_validation::is_existing_dir(self.dir_dst.as_ref())
    }
    fn with_valid_args(&self) -> io::Result<u8>
    {
        println!("TODO");
        Ok(0)
    }
}
