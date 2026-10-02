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