use clap::ArgMatches;

pub fn get_str_list_arg(args: &ArgMatches, id: &str) -> Vec<Box<String>>
{
    match args.get_many::<String>(id) {
        None => Vec::new(),
        Some(ss) => ss.map(|s| Box::new(s.clone())).collect(),
    }
}
