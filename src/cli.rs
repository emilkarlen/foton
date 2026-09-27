use clap::builder::*;
use crate::command::ExecutableCmd;
use crate::x2l::cli as x2l_cli;
use crate::enum_names::cli as enum_cli;
use crate::file_exts::cli as fe_cli;
use crate::sync_deletions::cli as sd_cli;

pub fn parse() -> Box<dyn ExecutableCmd>
{
    let matches = command().get_matches();
    let (name, args) = matches.subcommand().expect("subcommand is mandatory");
    match name {
        SUB_CMD_EXT2LOWER => x2l_cli::parse_cli_args(args),
        SUB_CMD_ENUM => enum_cli::parse_cli_args(args),
        SUB_CMD_FILE_EXTS => fe_cli::parse_cli_args(args),
        SUB_CMD_SYNC_DELETIONS => sd_cli::parse_cli_args(args),
        n => { panic!("unknown subcommand: {}", n) }
    }
}

const PROG_NAME: &str = env!("CARGO_BIN_NAME");

const VERSION: &str = env!("CARGO_PKG_VERSION");

const HELP_ABOUT: &str = "Tool for managing my photography files.";

const SUB_CMD_EXT2LOWER: &str = "ext2lower";
const SUB_CMD_ENUM: &str = "enum-names";
const SUB_CMD_FILE_EXTS: &str = "file-exts";
const SUB_CMD_SYNC_DELETIONS: &str = "sync-deletions";

fn command() -> Command
{
    Command::new(PROG_NAME)
        .version(VERSION)
        .about(HELP_ABOUT)
        .subcommand(x2l_cli::sub_cmd(SUB_CMD_EXT2LOWER))
        .subcommand(enum_cli::sub_cmd(SUB_CMD_ENUM))
        .subcommand(fe_cli::sub_cmd(SUB_CMD_FILE_EXTS))
        .subcommand(sd_cli::sub_cmd(SUB_CMD_SYNC_DELETIONS))
        .subcommand_required(true)
}
