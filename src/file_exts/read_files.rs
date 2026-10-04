use crate::common::path;
use crate::file_exts::config::ReadConfig;
use std::collections::HashMap;
use std::ffi::OsString;
use std::io;
use std::path::{Path, PathBuf};

pub fn execute(dirs: &Vec<PathBuf>, config: &ReadConfig) -> io::Result<HashMap<OsString, usize>> {
    let mut collection: HashMap<OsString, usize> = HashMap::new();
    for dir in dirs.iter() {
        read_files(dir, config, &mut collection)?;
    }
    Ok(collection)
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
                if config.include_hidden_sub_dirs || !path::is_hidden(name) {
                sub_dirs.push(path);
                    }
            }
        } else if f_type.is_file() {
            if let Some(file_name) = path.file_name() {
            if let (_, Some(ext)) = (config.split_stem_and_ext)(file_name) {
                collection.entry(OsString::from(ext)).and_modify(|n| *n += 1).or_insert(1);
            }
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
