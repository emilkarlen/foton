mod cli;
mod x2l;
mod enum_names;
mod file_exts;
mod command;
mod utils;
mod sync_deletions;
pub mod common;

fn main() -> std::process::ExitCode
{
    let cmd = cli::parse();

    cmd.execute()
}
