use clap::builder::*;
use clap::ArgMatches;
use crate::command::ExecutableCmd;
use super::{ext_filter, CmdConfig};
use std::path::Path;
use crate::enum_names::ext_filter::ExtensionsFilter;

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
    let opt_ext_include = Arg::new(OPT_INCLUDE_EXT_ID)
        .short('e')
        .long("ext")
        .help(OPT_INCLUDE_EXT_HELP)
        .action(ArgAction::Append);
    let opt_ext_exclude = Arg::new(OPT_SKIP_EXT_ID)
        .short('s')
        .long("skip")
        .help(OPT_SKIP_EXT_HELP)
        .action(ArgAction::Append);
    let arg_dir = Arg::new(OPT_DIR_ID)
        .required(true)
        .action(ArgAction::Set)
        .help(OPT_DIR_HELP);

    Command::new(name)
        .about(HELP_ABOUT)
        .after_help(HELP_AFTER_OPTIONS)
        .arg(opt_execute)
        .arg(opt_recursive)
        .arg(opt_ext_include)
        .arg(opt_ext_exclude)
        .group(
            ArgGroup::new("extensions")
                .multiple(false)
                .required(false)
                .args([OPT_INCLUDE_EXT_ID, OPT_SKIP_EXT_ID])
    ).arg(arg_dir)
}

pub fn parse_cli_args(args: &ArgMatches) -> Box<dyn ExecutableCmd>
{
    let d = args.get_one::<String>(OPT_DIR_ID).expect("mandatory");

    Box::from(CmdConfig {
        execute: args.get_flag(OPT_EXECUTE_ID),
        recursive: args.get_flag(OPT_RECURSIVE_ID),
        directory: Box::from(Path::new(&d)),
        extensions_filter: parse_extensions_filter(args),
    })
}

fn parse_extensions_filter(args: &ArgMatches) -> Box<dyn ExtensionsFilter>
{
    if args.contains_id(OPT_SKIP_EXT_ID) {
        Box::new(ext_filter::exclude(get_str_list_arg(args, OPT_SKIP_EXT_ID)))
    }
    else if args.contains_id(OPT_INCLUDE_EXT_ID) {
        Box::new(ext_filter::include(get_str_list_arg(args, OPT_INCLUDE_EXT_ID)))
    }
    else {
        Box::new(ext_filter::any())
    }
}

fn get_str_list_arg(args: &ArgMatches, id: &str) -> Vec<Box<String>>
{
    match args.get_many::<String>(id) {
        None => Vec::new(),
        Some(ss) => ss.map(|s| Box::new(s.clone())).collect(),
    }
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
const OPT_SKIP_EXT_ID: &str = "SKIP-EXT";
const OPT_SKIP_EXT_HELP: &str = "Rename only files not with given extensions (may be used multiple times)";
const OPT_INCLUDE_EXT_ID: &str = "INCLUDE-EXT";
const OPT_INCLUDE_EXT_HELP: &str = "Rename only files with given extensions (may be used multiple times)";
const OPT_DIR_HELP: &str = "The directory containing files to rename.";
const OPT_EXECUTE_SHORT: char = 'x';
const OPT_EXECUTE_HELP: &str = "Do execute the action (default is to run dry)";
