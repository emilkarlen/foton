use super::CmdConfig;
use crate::command::ExecutableCmd;
use crate::common::cli::get_str_list_arg_as_paths;
use crate::common::{cli, ext_filter_cli};
use crate::rename::config::NamingConfigCli;
use clap::builder::*;
use clap::{value_parser, ArgMatches};

pub fn sub_cmd(name: &'static str) -> Command
{
    let opt_start_num = Arg::new(OPT_START_NUM_ID)
        .long("start-num")
        .action(ArgAction::Set)
        .default_value("1")
        .value_parser(value_parser!(usize))
        .help(OPT_START_NUM_HELP);
    let opt_min_num_width = Arg::new(OPT_MIN_NUM_WIDTH_ID)
        .long("min-num-width")
        .action(ArgAction::Set)
        .default_value("1")
        .value_parser(value_parser!(usize))
        .help(OPT_MIN_NUM_WIDTH_HELP);
    let opt_format = Arg::new(OPT_FORMAT_ID)
        .long("format")
        .action(ArgAction::Set)
        .help(OPT_FORMAT_HELP);
    let opt_long_ext = Arg::new(cli::OPT_LONG_EXT_ID)
        .short('l')
        .long("long")
        .action(ArgAction::SetTrue)
        .help(cli::OPT_LONG_EXT_HELP);
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
        .action(ArgAction::Append)
        .help(OPT_DIR_HELP);

    let cmd = Command::new(name);
    let cmd = ext_filter_cli::add_ext_filter_options(cmd);
    cmd
        .about(HELP_ABOUT)
        .after_help(HELP_AFTER_OPTIONS)
        .arg(opt_start_num)
        .arg(opt_min_num_width)
        .arg(opt_format)
        .arg(opt_long_ext)
        .arg(opt_execute)
        .arg(opt_recursive)
        .arg(arg_dir)
}

pub fn parse_cli_args(args: &ArgMatches) -> Box<dyn ExecutableCmd>
{
    Box::from(CmdConfig {
        execute: args.get_flag(OPT_EXECUTE_ID),
        directories: get_str_list_arg_as_paths(args, OPT_DIR_ID),
        read_config: crate::common::read_files::ReadConfig {
            recursive: args.get_flag(OPT_RECURSIVE_ID),
            extensions_filter: ext_filter_cli::parse_extensions_filter(args),
            include_hidden_sub_dirs: true,
            split_stem_and_ext:  cli::get_stem_ext_splitter(args, cli::OPT_LONG_EXT_ID),
        },
        naming_config: NamingConfigCli {
            start_num: *args.get_one::<usize>(OPT_START_NUM_ID).unwrap(),
            min_width: *args.get_one::<usize>(OPT_MIN_NUM_WIDTH_ID).unwrap(),
            format: args.get_one(OPT_FORMAT_ID).map(String::clone),
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


const OPT_START_NUM_ID: &str = "START-NUM";
const OPT_START_NUM_HELP: &str = "Start numbering from given number";
const OPT_MIN_NUM_WIDTH_ID: &str = "MIN-WIDTH";
const OPT_MIN_NUM_WIDTH_HELP: &str = "Minimum width of number string";
const OPT_FORMAT_ID: &str = "FORMAT";
const OPT_FORMAT_HELP: &str = "Custom formatting using {NN} for file number and {stem} for original file name stem.";
const OPT_RECURSIVE_ID: &str = "recursive";
const OPT_RECURSIVE_HELP: &str = "Also rename files in sub-directories.";
const OPT_DIR_ID: &str = "DIR";
const OPT_DIR_HELP: &str = "The directories containing files to rename.";
const OPT_EXECUTE_ID: &str = "execute";
const OPT_EXECUTE_SHORT: char = 'x';
const OPT_EXECUTE_HELP: &str = "Do execute the action (default is to run dry)";
