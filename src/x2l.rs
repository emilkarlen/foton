pub mod cli;
mod err;
mod err_fmt;
mod renamer;
mod rename;
mod reporter;

use self::renamer::Renamer;
use self::reporter::Reporter;
use super::command::ExecutableCmd;

pub struct CmdConfig
{
    pub execute: bool,
}

impl ExecutableCmd for CmdConfig
{
    fn execute(&self) -> std::process::ExitCode
    {
        let mut reporter = Reporter::new();
        let renamer = Renamer::resolve(self.execute);

        rename::rename_files(std::io::stdin(), renamer,  &mut reporter)
    }
}
