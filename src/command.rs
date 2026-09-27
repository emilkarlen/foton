use std::io;

pub trait ExecutableCmd {
    fn validate_args(&self) -> Result<(), String>;
    fn with_valid_args(&self) -> io::Result<()>;
}
