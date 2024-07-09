pub mod cli;
mod err;
mod err_fmt;
mod renamer;
mod rename;
mod reporter;

use self::renamer::Renamer;
use self::reporter::Reporter;

pub struct CliArgs
{
    pub execute: bool,
}

pub fn sub_cmd_main(args: &CliArgs) -> std::process::ExitCode
{
    let mut reporter = Reporter::new();
    let renamer = Renamer::resolve(args.execute);

    rename::rename_files(std::io::stdin(), renamer,  &mut reporter)
   
}