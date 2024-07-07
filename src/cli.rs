use clap::Parser;


/// Read file names from stdin (one per line) and rename the file so that its extension is lowercase
#[derive(clap::Parser)]
pub struct CliArgs
{
    /// Do execute the action (default is to run dry)
    #[arg(short)]
    pub x: bool,
}

pub fn parse() -> CliArgs
{
    CliArgs::parse()
}
