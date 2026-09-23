use clap::builder::*;
use clap::ArgMatches;
use crate::command::ExecutableCmd;
use super::{CmdConfig};
use std::path::Path;
use crate::file_exts::config::{ReadConfig, ReportConfig};
use crate::file_exts::read_files;

pub fn sub_cmd(name: &'static str) -> Command
{
    let opt_recursive = Arg::new(OPT_RECURSIVE_ID)
        .short('r')
        .action(ArgAction::SetTrue)
        .help(OPT_RECURSIVE_HELP);
    let opt_ignore_hidden_sub_dirs = Arg::new(OPT_IGNORE_HIDDEN_SUB_DIRS_ID)
        .short('d')
        .action(ArgAction::SetTrue)
        .help(OPT_IGNORE_HIDDEN_SUB_DIRS_HELP);
    let opt_num_files = Arg::new(OPT_NUM_FILES_ID)
        .short('n')
        .action(ArgAction::SetTrue)
        .help(OPT_NUM_FILES_HELP);
    let opt_tot_num_files = Arg::new(OPT_TOT_NUM_FILES_ID)
        .short('N')
        .action(ArgAction::SetTrue)
        .help(OPT_TOT_NUM_FILES_HELP);
    let opt_sort_by_num = Arg::new(OPT_SORT_BY_NUM_ID)
        .short('s')
        .action(ArgAction::SetTrue)
        .help(OPT_SORT_BY_NUM_HELP);
    let opt_sort_reverse = Arg::new(OPT_SORT_REVERSE_ID)
        .short('R')
        .action(ArgAction::SetTrue)
        .help(OPT_SORT_REVERSE_HELP);
    let opt_long_ext = Arg::new(OPT_LONG_EXT_ID)
        .short('l')
        .action(ArgAction::SetTrue)
        .help(OPT_LONG_EXT_HELP);
    let arg_dir = Arg::new(OPT_DIR_ID)
        .required(true)
        .action(ArgAction::Append)
        .help(OPT_DIR_HELP);

    Command::new(name)
        .about(HELP_ABOUT)
        .arg(opt_recursive)
        .arg(opt_ignore_hidden_sub_dirs)
        .arg(opt_long_ext)
        .arg(opt_num_files)
        .arg(opt_tot_num_files)
        .arg(opt_sort_by_num)
        .arg(opt_sort_reverse)
        .arg(arg_dir)
}

pub fn parse_cli_args(args: &ArgMatches) -> Box<dyn ExecutableCmd>
{
    Box::from(CmdConfig {
        read_config: ReadConfig {
            recursive: args.get_flag(OPT_RECURSIVE_ID),
            include_hidden_sub_dirs: !args.get_flag(OPT_IGNORE_HIDDEN_SUB_DIRS_ID),
            get_ext:  if args.get_flag(OPT_LONG_EXT_ID) {
                read_files::get_long_ext
            }
            else {
                read_files::get_short_ext
            },
        },
        report_config: ReportConfig {
            num_files: args.get_flag(OPT_NUM_FILES_ID),
            tot_num_files: args.get_flag(OPT_TOT_NUM_FILES_ID),
            sort_on_num_ext: args.get_flag(OPT_SORT_BY_NUM_ID),
            sort_reverse: args.get_flag(OPT_SORT_REVERSE_ID),
        },
        directories: get_str_list_arg_as_paths(args, OPT_DIR_ID),
    })
}

fn get_str_list_arg_as_paths(args: &ArgMatches, id: &str) -> Vec<Box<Path>>
{
    match args.get_many::<String>(id) {
        None => Vec::new(),
        Some(ss) => ss.map(|s| Box::from(Path::new(&s))).collect(),
    }
}

const HELP_ABOUT: &str =
"Print all unique extensions of regular files, sorted.";

const OPT_RECURSIVE_ID: &str = "recursive";
const OPT_RECURSIVE_HELP: &str = "Include files in sub directories";
const OPT_IGNORE_HIDDEN_SUB_DIRS_ID: &str = "hidden-sub-dirs";
const OPT_IGNORE_HIDDEN_SUB_DIRS_HELP: &str = "Ignore files in hidden sub directories";
const OPT_LONG_EXT_ID: &str = "long-ext";
const OPT_LONG_EXT_HELP: &str = "Treat everything after the first (non-initial) dot as the extension";
const OPT_NUM_FILES_ID: &str = "num-files";
const OPT_NUM_FILES_HELP: &str = "Print the number of files with each extension";
const OPT_TOT_NUM_FILES_ID: &str = "tot-num-files";
const OPT_TOT_NUM_FILES_HELP: &str = "Print the total number of files";
const OPT_SORT_BY_NUM_ID: &str = "sort-by-num";
const OPT_SORT_BY_NUM_HELP: &str = "Sort by number of files per extension";
const OPT_SORT_REVERSE_ID: &str = "sort-reverse";
const OPT_SORT_REVERSE_HELP: &str = "Reverse sort";
const OPT_DIR_ID: &str = "DIR";
const OPT_DIR_HELP: &str = "The directories in which to look for files";
