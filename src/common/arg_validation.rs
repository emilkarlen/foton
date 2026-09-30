use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use crate::common::read_files::PathWithName;

pub fn is_existing_dir(p: &Path) -> Result<(), String>
{
    if !p.is_dir() {
        let s = format!("not a dir: {}\n", p.display());
        Err(s)
    } else {
        Ok(())
    }
}
pub fn has_name(p: &Path) -> Result<&OsStr, String>
{
    if let Some(name) = p.file_name() {
        Ok(name)
    }
    else {
        Err(format!("path does not have a new: {}", p.display()))
    }
}

pub fn is_path_with_name(path: &PathBuf, dir_name: &'static str) -> Result<PathWithName, String>
{
    match PathWithName::from(path.clone()) {
        Some(p) => Ok(p),
        None => Err(format!("path does not have a name ({}): {}", dir_name, path.display())),
    }
}
