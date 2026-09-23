pub mod cli;
mod read_files;
mod report;
mod config;

use std::io;
use std::io::Write;
use std::path::Path;
use std::process::ExitCode;
use crate::command::ExecutableCmd;
use crate::file_exts::config::{ReportConfig, ReadConfig};

pub struct CmdConfig
{
    pub read_config: ReadConfig,
    pub report_config: ReportConfig,
    pub directories: Vec<Box<Path>>,
}

const EXIT_INVALID_ARG: u8 = 2;

impl ExecutableCmd for CmdConfig
{
    fn execute(&self) -> ExitCode
    {
        for path in self.directories.iter() {
            if !path.is_dir() {
                eprintln!("not a dir: {}\n", path.display());
                return ExitCode::from(EXIT_INVALID_ARG);
            }
        }
        match self.with_valid_args() {
            Ok(exit_code) => ExitCode::from(exit_code),
            Err(e) => {
                io::stderr().write_fmt(format_args!("{}\n", e)).expect("Could not write to stderr");
                ExitCode::FAILURE
            }
        }
    }
}

impl CmdConfig
{
    fn with_valid_args(&self) -> io::Result<u8>
    {
        let mut extensions = read_files::execute(&self.directories, &self.read_config)?;
        report::execute(&self.report_config, &mut extensions);
        Ok(0)
    }
}
