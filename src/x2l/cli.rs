use clap::builder::*;
use clap::ArgMatches;

use super::CliArgs;

pub fn sub_cmd(name: &'static str) -> Command
{
    Command::new(name)
    .about(X2L_HELP_ABOUT)
    .after_help(X2L_HELP_AFTER_OPTIONS)
    .arg(
        Arg::new(OPT_EXECUTE_ID)
        .short(OPT_EXECUTE_SHORT)
        .action(ArgAction::SetTrue)
        .help(OPT_EXECUTE_HELP)
    )
}

pub fn parse_cli_args(args: &ArgMatches) -> CliArgs
{
    CliArgs {
        execute: args.get_flag(OPT_EXECUTE_ID),
    }
}

const X2L_HELP_ABOUT: &str =
"Read file names from stdin (one per line) \
and rename the file so that its extension is lowercase.";
const X2L_HELP_AFTER_OPTIONS: &str =
"Renamings are reported on stdout (one per line). \
Errors and files not renamed are reported on stderr.";

const OPT_EXECUTE_ID: &str = "execute";
const OPT_EXECUTE_SHORT: char = 'x';
const OPT_EXECUTE_HELP: &str = "Do execute the action (default is to run dry)";
