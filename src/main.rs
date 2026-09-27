use std::process::ExitCode;
use crate::common::cli_exit::EXIT_INVALID_ARG;

mod cli;
mod x2l;
mod enum_names;
mod file_exts;
mod command;
mod utils;
mod sync_deletions;
pub mod common;

fn main() -> std::process::ExitCode
{
    let cmd = cli::parse();

    if let Err(err_msg) = cmd.validate_args() {
        eprintln!("{}", err_msg);
        return ExitCode::from(EXIT_INVALID_ARG);
    }
    match cmd.with_valid_args() {
        Ok(_) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("{}\n", e);
            ExitCode::FAILURE
        }
    }
}
