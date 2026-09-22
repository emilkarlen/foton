mod cli;
mod x2l;
mod enum_names;
mod command;

fn main() -> std::process::ExitCode
{
    let cmd = cli::parse();

    cmd.execute()
}
