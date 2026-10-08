use super::CmdConfig;
use crate::command::ExecutableCmd;
use crate::common::read_files::config_cli as read_config_cli;
use crate::sync_deletions::config::ProcessConfig;
use clap::builder::*;
use clap::ArgMatches;
use std::path::PathBuf;

pub fn sub_cmd(name: &'static str) -> Command
{
    let opt_execute = Arg::new(OPT_EXECUTE_ID)
        .short(OPT_EXECUTE_SHORT)
        .action(ArgAction::SetTrue)
        .help(OPT_EXECUTE_HELP);
    let opt_move = Arg::new(OPT_MOVE_ID)
        .long("move")
        .action(ArgAction::Set)
        .help(OPT_MOVE_HELP);
    let arg_dir_src = Arg::new(OPT_DIR_SRC_ID)
        .required(true)
        .action(ArgAction::Set)
        .help(OPT_DIR_SRC_HELP);
    let arg_dir_dst = Arg::new(OPT_DIR_DST_ID)
        .required(true)
        .action(ArgAction::Set)
        .help(OPT_DIR_DST_HELP);

    let cmd = Command::new(name);
    let cmd = read_config_cli::add_read_config_options(cmd);
    cmd
        .about(HELP_ABOUT)
        .after_help(HELP_AFTER_OPTIONS)
        .arg(opt_execute)
        .arg(opt_move)
        .arg(arg_dir_src)
        .arg(arg_dir_dst)
}

pub fn parse_cli_args(args: &ArgMatches) -> Box<dyn ExecutableCmd>
{
    let dir_src = args.get_one::<String>(OPT_DIR_SRC_ID).expect("mandatory");
    let dir_dst = args.get_one::<String>(OPT_DIR_DST_ID).expect("mandatory");
    let dir_move = args.get_one::<String>(OPT_MOVE_ID).map(PathBuf::from);

    Box::from(CmdConfig {
        process_config: ProcessConfig {
        execute: args.get_flag(OPT_EXECUTE_ID),
        move_to: dir_move,
    },
        dir_src: PathBuf::from(&dir_src),
        dir_dst: PathBuf::from(&dir_dst),
        read_config: read_config_cli::parse_args_matches(args),
    })
}

const HELP_ABOUT: &str =
    "Deletes files in DST that does not have a counterpart in SRC";
const HELP_AFTER_OPTIONS: &str =
    "Deletes every file in DST that does not have a corresponding file in SRC.\n\n
A file DST/X.EXT has a corresponding file in SRC iff:\n
  there exists a file matching SRC/X.*";

const OPT_EXECUTE_ID: &str = "execute";
const OPT_MOVE_ID: &str = "MOVE-TO-DIR";
const OPT_MOVE_HELP: &str = "Moves deleted files from DST-DIR to the given directory, instead of deleting (must be on same file system)";

const OPT_DIR_SRC_ID: &str = "SRC-DIR";
const OPT_DIR_DST_ID: &str = "DST-DIR";
const OPT_DIR_SRC_HELP: &str = "The directory where files have been deleted";
const OPT_DIR_DST_HELP: &str = "The directory in which to delete files";
const OPT_EXECUTE_SHORT: char = 'x';
const OPT_EXECUTE_HELP: &str = "Do execute the action (default is to run dry)";
