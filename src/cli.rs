use clap::builder::*;
use crate::x2l::cli as x2l_cli;
use crate::x2l::CliArgs;

pub fn parse() -> CliArgs
{
    let matches = command().get_matches();

    let x2l_matches = matches.subcommand_matches(SUB_CMD_EXT2LOWER)
        .expect("The one and only sub command");

    x2l_cli::parse_cli_args(&x2l_matches)
}

const PROG_NAME: &str = env!("CARGO_BIN_NAME");

const VERSION: &str = env!("CARGO_PKG_VERSION");

const HELP_ABOUT: &str = "Tool for managing my photography files.";

const SUB_CMD_EXT2LOWER: &str = "ext2lower";

fn command() -> Command
{
    Command::new(PROG_NAME)
        .version(VERSION)
        .about(HELP_ABOUT)
        .subcommand(x2l_cli::sub_cmd(SUB_CMD_EXT2LOWER))
        .subcommand_required(true)
}
