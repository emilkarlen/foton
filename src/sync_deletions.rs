pub mod cli;

use crate::command::ExecutableCmd;
use crate::common::arg_validation;
use crate::common::ext_filter::ExtensionsFilter;
use std::io;
use std::path::Path;

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
    fn validate_args(&self) -> Result<(), String>
    {
        arg_validation::is_existing_dir(self.dir_src.as_ref())?;
        arg_validation::is_existing_dir(self.dir_src.as_ref())?;
        arg_validation::is_existing_dir(self.dir_dst.as_ref())
    }
    fn with_valid_args(&self) -> io::Result<()>
    {
        println!("TODO");
        Ok(())
    }
}
