use clap::Parser;

/// Read file names from stdin (one per line) and rename the file so that its extension is lowercase
#[derive(Parser)]
struct Cli
{
    /// Do execute the action (default is to run dry)
    #[arg(short)]
    x: bool,
}

fn main()
{
    let args = Cli::parse();

    println!("mode: {:?}", args.x);

    println!("Hello, world!");
}
