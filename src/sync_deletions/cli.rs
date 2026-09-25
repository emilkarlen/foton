use super::CmdConfig;
use crate::command::ExecutableCmd;
use crate::common::ext_filter_cli;
use crate::common::read_files::ReadConfig;
use clap::builder::*;
use clap::ArgMatches;
use std::path::PathBuf;

pub fn sub_cmd(name: &'static str) -> Command
{
    let opt_execute = Arg::new(OPT_EXECUTE_ID)
        .short(OPT_EXECUTE_SHORT)
        .action(ArgAction::SetTrue)
        .help(OPT_EXECUTE_HELP);
    let opt_recursive = Arg::new(OPT_RECURSIVE_ID)
        .short('r')
        .action(ArgAction::SetTrue)
        .help(OPT_RECURSIVE_HELP);
    let opt_ignore_hidden_sub_dirs = Arg::new(OPT_IGNORE_HIDDEN_SUB_DIRS_ID)
        .short('d')
        .action(ArgAction::SetTrue)
        .help(OPT_IGNORE_HIDDEN_SUB_DIRS_HELP);
    let arg_dir_src = Arg::new(OPT_DIR_SRC_ID)
        .required(true)
        .action(ArgAction::Set)
        .help(OPT_DIR_SRC_HELP);
    let arg_dir_dst = Arg::new(OPT_DIR_DST_ID)
        .required(true)
        .action(ArgAction::Set)
        .help(OPT_DIR_DST_HELP);

    let cmd = Command::new(name);
    let cmd = ext_filter_cli::add_ext_filter_options(cmd);
    cmd
        .about(HELP_ABOUT)
        .after_help(HELP_AFTER_OPTIONS)
        .arg(opt_execute)
        .arg(opt_recursive)
        .arg(opt_ignore_hidden_sub_dirs)
        .arg(arg_dir_src)
        .arg(arg_dir_dst)
}

pub fn parse_cli_args(args: &ArgMatches) -> Box<dyn ExecutableCmd>
{
    let dir_src = args.get_one::<String>(OPT_DIR_SRC_ID).expect("mandatory");
    let dir_dst = args.get_one::<String>(OPT_DIR_DST_ID).expect("mandatory");

    Box::from(CmdConfig {
        execute: args.get_flag(OPT_EXECUTE_ID),
        dir_src: PathBuf::from(&dir_src),
        dir_dst: PathBuf::from(&dir_dst),
        read_config: ReadConfig {
            recursive: true, // args.get_flag(OPT_RECURSIVE_ID),
            include_hidden_sub_dirs: !args.get_flag(OPT_IGNORE_HIDDEN_SUB_DIRS_ID),
            extensions_filter: ext_filter_cli::parse_extensions_filter(args),
        }
    })
}

const HELP_ABOUT: &str =
    "Deletes files in DST that does not have a counterpart in SRC";
const HELP_AFTER_OPTIONS: &str =
    "Deletes every file in DST that does not have a corresponding file in SRC.\n\n
A file DST/X.EXT has a corresponding file in SRC iff:\n
  there exists a file matching SRC/X.*";

const OPT_EXECUTE_ID: &str = "execute";

const OPT_RECURSIVE_ID: &str = "recursive";
const OPT_RECURSIVE_HELP: &str = "Also rename files in sub-directories.";
const OPT_DIR_SRC_ID: &str = "SRC-DIR";
const OPT_DIR_DST_ID: &str = "DST-DIR";
const OPT_DIR_SRC_HELP: &str = "The directory where files have been deleted";
const OPT_DIR_DST_HELP: &str = "The directory in which to deletions files";
const OPT_EXECUTE_SHORT: char = 'x';
const OPT_EXECUTE_HELP: &str = "Do execute the action (default is to run dry)";
const OPT_IGNORE_HIDDEN_SUB_DIRS_ID: &str = "hidden-sub-dirs";
const OPT_IGNORE_HIDDEN_SUB_DIRS_HELP: &str = "Ignore files in hidden sub directories";
