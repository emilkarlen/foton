use crate::common::ext_filter::ExtensionsFilter;
use super::common::{DirContents, FnInfo};
use super::config::ReadConfig;
use std::collections::HashMap;
use std::fs;
use std::fs::FileType;
use std::io;
use std::ops::Deref;
use std::path::Path;
use crate::utils::from_os_str;

pub fn rev_sorted_file_infos(dir: &Path, config: &ReadConfig) -> io::Result<DirContents<FnInfo>>
{
    let (sub_dir_names, files) = rev_sorted_file_infos_non_rec(dir, config.extensions_filter.deref())?;
    let mut sub_dir_names = sub_dir_names;
    let mut sub_dirs = Vec::with_capacity(sub_dir_names.len());
    if config.recursive {
        for sub_dir_name in sub_dir_names.drain(..).rev() {
            let sub_dir_path = dir.join(Path::new(&sub_dir_name));
            let sub_dir_contents = rev_sorted_file_infos(&sub_dir_path, config)?;
            sub_dirs.push(sub_dir_contents);
        }
    }
    Ok(DirContents{dir: Box::from(dir), sub_dirs, files, })
}
fn rev_sorted_file_infos_non_rec(dir: &Path, extensions_filter: &dyn ExtensionsFilter) -> io::Result<(Vec<String>, Vec<FnInfo>)>
{
    let (sub_dirs, files) = read_files(dir, extensions_filter)?;
    let mut sorted_sub_dirs = sub_dirs;
    sorted_sub_dirs.sort();
    let mut rev_sorted_fnis: Vec<_> = files.into_iter().map(FnInfo::from).collect();
    rev_sorted_fnis.sort_by(|x, y| y.stem.cmp(&x.stem));
    Ok((sorted_sub_dirs, rev_sorted_fnis))
}

fn read_files(dir: &Path, extensions_filter: &dyn ExtensionsFilter) -> io::Result<(Vec<String>, HashMap<String, Vec<String>>)>
{
    let mut sub_dirs: Vec<String> = Vec::new();
    let mut files: HashMap<String, Vec<String>> = HashMap::new();
    let entries = dir.read_dir()?;
    for mb_entry in entries {
        let entry = mb_entry?;
        let f_type = entry.file_type()?;
        let mb_dof = DirOrFile::from(&f_type, &entry);
        if let Some(dof) = mb_dof {
            match dof {
                DirOrFile::ADir(dir_name) => { sub_dirs.push(dir_name) },
                DirOrFile::AFile(se) => {
                    if !extensions_filter.accept(&se.ext.as_str()) {
                        continue;
                    }
                    match files.get_mut(&se.stem) {
                        None => {
                            files.insert(se.stem, vec![se.ext]);
                        }
                        Some(exts) => {
                            exts.push(se.ext);
                        }
                    }
                }
            }
        }
    }
    for exts in files.values_mut() {
        exts.sort();
    }
    Ok((sub_dirs, files))
}

struct StemAndExt
{
    stem: String,
    ext: String,
}

enum DirOrFile
{
    ADir(String),
    AFile(StemAndExt),
}

impl DirOrFile
{
    fn from(f_type: &FileType, entry: &fs::DirEntry) -> Option<DirOrFile>
    {
        if f_type.is_dir() {
            let path = entry.path();
            let file_name = path.file_name()?;
            Some(DirOrFile::ADir(from_os_str(file_name)))
        }
        else if f_type.is_file() {
            let x = StemAndExt::from(entry)?;
            Some(DirOrFile::AFile(x))
        }
        else {
            None
        }
    }

}
impl StemAndExt
{
    fn from(entry: &fs::DirEntry) -> Option<StemAndExt>
    {
        let path = entry.path();
        let stem = path.file_stem()?;
        let ext = path.extension()?;
        Some(StemAndExt {
            stem: from_os_str(stem),
            ext: from_os_str(ext)
        }
        )
    }
}
