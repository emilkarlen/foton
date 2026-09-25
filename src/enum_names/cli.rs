use super::CmdConfig;
use crate::command::ExecutableCmd;
use crate::common::ext_filter_cli;
use crate::enum_names::config::ReadConfig;
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
    let arg_dir = Arg::new(OPT_DIR_ID)
        .required(true)
        .action(ArgAction::Set)
        .help(OPT_DIR_HELP);

    let cmd = Command::new(name);
    let cmd = ext_filter_cli::add_ext_filter_options(cmd);
    cmd
        .about(HELP_ABOUT)
        .after_help(HELP_AFTER_OPTIONS)
        .arg(opt_execute)
        .arg(opt_recursive)
        .arg(arg_dir)
}

pub fn parse_cli_args(args: &ArgMatches) -> Box<dyn ExecutableCmd>
{
    let d = args.get_one::<String>(OPT_DIR_ID).expect("mandatory");

    Box::from(CmdConfig {
        execute: args.get_flag(OPT_EXECUTE_ID),
        directory: PathBuf::from(&d),
        read_config: ReadConfig {
            recursive: args.get_flag(OPT_RECURSIVE_ID),
            extensions_filter: ext_filter_cli::parse_extensions_filter(args),
        }
    })
}

const HELP_ABOUT: &str =
"Renames the the basename of regular files in a given directory, \
to 01.EXT, 02.EXT, ...";
const HELP_AFTER_OPTIONS: &str =
    "Files are enumerated in alphabetic order.\n\n\
    Files without an extension are not renamed.\n\n\
    Recursive application first enumerates the files in the dir itself,\n\
    and then in sub directories (in alphabetic order).\n\n\
    Renames are reported on stdout.";

const OPT_EXECUTE_ID: &str = "execute";

const OPT_RECURSIVE_ID: &str = "recursive";
const OPT_RECURSIVE_HELP: &str = "Also rename files in sub-directories.";
const OPT_DIR_ID: &str = "DIR";
const OPT_DIR_HELP: &str = "The directory containing files to rename.";
const OPT_EXECUTE_SHORT: char = 'x';
const OPT_EXECUTE_HELP: &str = "Do execute the action (default is to run dry)";
