use std::path::PathBuf;

pub struct ProcessConfig
{
    pub execute: bool,
    pub move_to: Option<PathBuf>,
}
