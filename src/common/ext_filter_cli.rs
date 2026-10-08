use clap::builder::*;
use clap::ArgMatches;
use super::ext_filter;
use super::cli;


pub fn add_ext_filter_options(cmd: Command) -> Command
{
    let opt_ext_include = Arg::new(OPT_INCLUDE_EXT_ID)
        .short('i')
        .long("include")
        .value_name("EXTENSIONS")
        .help(OPT_INCLUDE_EXT_HELP)
        .action(ArgAction::Append)
        .value_delimiter(',');
    let opt_ext_exclude = Arg::new(OPT_EXCLUDE_EXT_ID)
        .short('e')
        .long("exclude")
        .value_name("EXTENSIONS")
        .help(OPT_EXCLUDE_EXT_HELP)
        .action(ArgAction::Append)
        .value_delimiter(',');

    let cmd = cmd.arg(opt_ext_exclude);
    let cmd = cmd.arg(opt_ext_include);
    let cmd = cmd.group(
        ArgGroup::new("extensions")
            .multiple(false)
            .required(false)
            .args([OPT_INCLUDE_EXT_ID, OPT_EXCLUDE_EXT_ID]));

    cmd
}

pub fn parse_extensions_filter(args: &ArgMatches) -> Box<dyn ext_filter::ExtensionsFilter>
{
    if args.contains_id(OPT_EXCLUDE_EXT_ID) {
        Box::new(ext_filter::exclude(cli::get_str_list_arg(args, OPT_EXCLUDE_EXT_ID)))
    }
    else if args.contains_id(OPT_INCLUDE_EXT_ID) {
        Box::new(ext_filter::include(cli::get_str_list_arg(args, OPT_INCLUDE_EXT_ID)))
    }
    else {
        Box::new(ext_filter::any())
    }
}

const OPT_EXCLUDE_EXT_ID: &str = "SKIP-EXT";
const OPT_EXCLUDE_EXT_HELP: &str = "Process only files not with given extensions (may be used multiple times)";
const OPT_INCLUDE_EXT_ID: &str = "INCLUDE-EXT";
const OPT_INCLUDE_EXT_HELP: &str = "Process only files with given extensions (may be used multiple times)";
