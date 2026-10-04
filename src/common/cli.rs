use super::path;
use crate::common::read_files::StemExtSplitter;
use clap::ArgMatches;
use std::path::PathBuf;

pub fn get_str_list_arg(args: &ArgMatches, id: &str) -> Vec<Box<String>>
{
    match args.get_many::<String>(id) {
        None => Vec::new(),
        Some(ss) => ss.map(|s| Box::new(s.clone())).collect(),
    }
}

pub fn get_str_list_arg_as_paths(args: &ArgMatches, id: &str) -> Vec<PathBuf>
{
    match args.get_many::<String>(id) {
        None => Vec::new(),
        Some(ss) => ss.map(PathBuf::from).collect(),
    }
}

pub fn get_stem_ext_splitter(
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

pub const OPT_LONG_EXT_ID: &str = "long-ext";
pub const OPT_LONG_EXT_HELP: &str = "Treat everything after the first (non-initial) dot as the extension";
