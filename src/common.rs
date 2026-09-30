pub mod ext_filter;
pub mod ext_filter_cli;
pub mod cli;
pub mod cli_exit;
pub mod arg_validation;
pub mod dir_contents;
pub mod read_files;
pub mod fs;

pub const PROG_NAME: &str = env!("CARGO_BIN_NAME");
