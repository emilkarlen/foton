use crate::common::ext_filter_cli::add_ext_filter_options;
use crate::common::read_files::types::StemExtSplitter;
use crate::common::read_files::ReadConfig;
use crate::common::{cli, ext_filter_cli, path};
use clap::builder::*;
use clap::ArgMatches;

pub fn add_read_config_options(cmd: Command) -> Command
{
    let opt_recursive = Arg::new(OPT_RECURSIVE_ID)
        .short('r')
        .action(ArgAction::SetTrue)
        .help(OPT_RECURSIVE_HELP);
    let opt_ignore_hidden_sub_dirs = Arg::new(OPT_IGNORE_HIDDEN_SUB_DIRS_ID)
        .short('d')
        .action(ArgAction::SetTrue)
        .help(OPT_IGNORE_HIDDEN_SUB_DIRS_HELP);
    let opt_long_exts = Arg::new(OPT_LONG_EXT_ID)
        .short('l')
        .long("long")
        .action(ArgAction::SetTrue)
        .help(OPT_LONG_EXT_HELP);

    let cmd = cmd.arg(opt_recursive)
        .arg(opt_ignore_hidden_sub_dirs)
        .arg(opt_long_exts);
    let cmd = add_ext_filter_options(cmd);
    cmd
}

pub fn parse_args_matches(args: &ArgMatches) -> ReadConfig
{
    ReadConfig {
        recursive: args.get_flag(OPT_RECURSIVE_ID),
        include_hidden_sub_dirs: !args.get_flag(OPT_IGNORE_HIDDEN_SUB_DIRS_ID),
        extensions_filter: ext_filter_cli::parse_extensions_filter(args),
        split_stem_and_ext: get_stem_ext_splitter(args, cli::OPT_LONG_EXT_ID),
    }

}

fn get_stem_ext_splitter(
    args: &ArgMatches,
    id: &str,
) -> StemExtSplitter
{
    if args.get_flag(id) {
        path::split_longest_ext
    } else {
        path::split_shortest_ext
    }
}

const OPT_RECURSIVE_ID: &str = "recursive";
const OPT_RECURSIVE_HELP: &str = "Also delete files in sub-directories";
const OPT_IGNORE_HIDDEN_SUB_DIRS_ID: &str = "hidden-sub-dirs";
const OPT_IGNORE_HIDDEN_SUB_DIRS_HELP: &str = "Ignore files in hidden sub directories";

pub const OPT_LONG_EXT_ID: &str = "long-ext";
pub const OPT_LONG_EXT_HELP: &str = "Treat everything after the first (non-initial) dot as the extension";
