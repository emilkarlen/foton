mod cli;
mod x2l;

fn main() -> std::process::ExitCode
{
    let args = cli::parse();

    x2l::sub_cmd_main(&args)
}
