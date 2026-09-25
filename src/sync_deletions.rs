pub mod cli;
pub mod main;

use crate::command::ExecutableCmd;
use crate::common::arg_validation;
use crate::common::read_files::{PathWithName, ReadConfig};
use std::io;
use std::path::PathBuf;

pub struct CmdConfig
{
    pub execute: bool,
    pub dir_src: PathBuf,
    pub dir_dst: PathBuf,
    pub read_config: ReadConfig,
}

impl ExecutableCmd for CmdConfig
{
    fn validate_args(&self) -> Result<(), String>
    {
        arg_validation::is_existing_dir(self.dir_src.as_ref())?;
        arg_validation::is_existing_dir(self.dir_dst.as_ref())
    }
    fn with_valid_args(&self) -> io::Result<()>
    {
        let dir_src = PathWithName::from(self.dir_src.clone()).unwrap();
        let dir_dst = PathWithName::from(self.dir_dst.clone()).unwrap();
        main::with_valid_args(self.execute, dir_src, dir_dst, &self.read_config)
    }
}
