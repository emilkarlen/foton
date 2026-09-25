use std::collections::HashMap;
use std::ffi::{OsStr, OsString};
use std::io;
use std::path::{Path, PathBuf};
use crate::file_exts::config::ReadConfig;
use crate::utils;

pub fn execute(dirs: &Vec<Box<Path>>, config: &ReadConfig) -> io::Result<HashMap<OsString, usize>> {
    let mut collection: HashMap<OsString, usize> = HashMap::new();
    for dir in dirs.iter() {
        read_files(dir, config, &mut collection)?;
    }
    Ok(collection)
}

pub fn get_short_ext(path: &Path) -> Option<OsString>
{
    let ext = path.extension()?;
    Some(ext.to_os_string())
}
pub fn get_long_ext(path: &Path) -> Option<OsString>
{
    // A BIT UNSAFE
    // Assumes that the dot character is represented by one byte.
    //
    // Might not work on Windows.

    let name = path.file_name()?.to_str()?;
    let prefix = path.file_prefix()?.to_str()?;
    if name.len() == prefix.len() {
        None
    }
    else {
        // UNSAFETY IS HERE!
        let ret_val_str = &name[prefix.len()+1..];
        let ret_val_oss = OsString::from(ret_val_str);
        Some(ret_val_oss)
    }
}

fn read_files(dir: &Path, config: &ReadConfig, collection: &mut HashMap<OsString, usize>) -> io::Result<()>
{
    let mut sub_dirs: Vec<PathBuf> = Vec::new();
    let entries = dir.read_dir()?;
    for mb_entry in entries {
        let entry = mb_entry?;
        let path = entry.path();
        let f_type = entry.file_type()?;
        if f_type.is_dir() {
            if let Some(name) = path.file_name() {
                if config.include_hidden_sub_dirs || !is_hidden(name) {
                sub_dirs.push(path);
                    }
            }
        } else if f_type.is_file() {
            if let Some(ext) = (config.get_ext)(&path) {
                collection.entry(ext).and_modify(|n| *n += 1).or_insert(1);
            }
        }
    }
    if config.recursive {
        for sub_dir in sub_dirs.iter() {
            read_files(sub_dir, config, collection)?;
        }
    }
    Ok(())
}

fn is_hidden(name: &OsStr) -> bool
{
    if let Some(ch) = utils::from_os_str(name).chars().next() {
        ch == '.'
    } else {
        // string is empty
        true
    }
}
