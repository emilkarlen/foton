mod cli;
mod err;
mod err_fmt;
mod renamer;
mod rename;
mod reporter;

use crate::renamer::Renamer;
use crate::reporter::Reporter;


fn main() -> std::process::ExitCode
{
    let args = cli::parse();

    let mut reporter = Reporter::new();
    let renamer = Renamer::resolve(args.x);

    rename::rename_files(std::io::stdin(), renamer,  &mut reporter)
}
