use std::process::ExitCode;
use crate::command::{CmdError, ExeError};
use crate::common::cli_exit::{EXIT_INVALID_ARG, EXIT_OTHER};
use crate::common::PROG_NAME;

mod cli;
mod x2l;
mod rename;
mod file_exts;
mod command;
mod utils;
mod sync_deletions;
pub mod common;

fn main() -> ExitCode
{
    let cmd = cli::parse();

    match cmd.execute() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            match e {
                CmdError::ArgsValidationError(err_msg) => {
                    eprintln!("{}: {}", PROG_NAME, err_msg);
                    ExitCode::from(EXIT_INVALID_ARG)
                },
                CmdError::ExecutionError(e) => {
                    match e {
                        ExeError::UnableToExecute(msg) => {
                            eprintln!("{}: Failure: {}", PROG_NAME, msg.reason);
                            msg.details.iter().for_each(|s| eprintln!("{s}"));
                            ExitCode::from(EXIT_OTHER)
                        }
                        ExeError::OsError(e) => {
                            eprintln!("{}: {}", PROG_NAME, e);
                            ExitCode::FAILURE
                        },
                    }
                }
            }
        }
    }
}
