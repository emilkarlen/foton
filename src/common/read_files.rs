use crate::common::dir_contents::DirContents;
use crate::common::ext_filter::ExtensionsFilter;
use crate::utils::from_os_str;
use std::collections::HashMap;
use std::ffi::OsString;
use std::fs;
use std::fs::FileType;
use std::io;
use std::ops::Deref;
use std::path::{Path, PathBuf};

pub struct FnInfo
{
    pub stem: OsString,
    pub extensions: Vec<OsString>,
}

impl FnInfo
{
    pub fn from(x: (OsString, Vec<OsString>)) -> FnInfo
    {
        FnInfo {
            stem: x.0,
            extensions: x.1,
        }
    }
}

pub struct ReadConfig
{
    pub recursive: bool,
    pub extensions_filter: Box<dyn ExtensionsFilter>,
}

pub fn rev_sorted_file_infos(dir: PathBuf, config: &ReadConfig) -> io::Result<DirContents<FnInfo>>
{
    let (sub_dir_names, files) = rev_sorted_file_infos_non_rec(&dir, config.extensions_filter.deref())?;
    let mut sub_dir_names = sub_dir_names;
    let mut sub_dirs = Vec::with_capacity(sub_dir_names.len());
    if config.recursive {
        for sub_dir_path in sub_dir_names.drain(..).rev() {
            let sub_dir_contents = rev_sorted_file_infos(sub_dir_path, config)?;
            sub_dirs.push(sub_dir_contents);
        }
    }
    Ok(DirContents{dir, sub_dirs, files, })
}
fn rev_sorted_file_infos_non_rec(dir: &PathBuf, extensions_filter: &dyn ExtensionsFilter) -> io::Result<(Vec<PathBuf>, Vec<FnInfo>)>
{
    let (sub_dirs, files) = read_files(dir, extensions_filter)?;
    let mut sorted_sub_dirs = sub_dirs;
    sorted_sub_dirs.sort();
    let mut rev_sorted_fnis: Vec<_> = files.into_iter().map(FnInfo::from).collect();
    rev_sorted_fnis.sort_by(|x, y| y.stem.cmp(&x.stem));
    Ok((sorted_sub_dirs, rev_sorted_fnis))
}

fn read_files(dir: &Path, extensions_filter: &dyn ExtensionsFilter) -> io::Result<(Vec<PathBuf>, HashMap<OsString, Vec<OsString>>)>
{
    let mut sub_dirs: Vec<PathBuf> = Vec::new();
    let mut files: HashMap<OsString, Vec<OsString>> = HashMap::new();
    let entries = dir.read_dir()?;
    for mb_entry in entries {
        let entry = mb_entry?;
        let f_type = entry.file_type()?;
        let mb_dof = DirOrFile::from(&f_type, &entry);
        if let Some(dof) = mb_dof {
            match dof {
                DirOrFile::ADir(path) => { sub_dirs.push(path) }
                DirOrFile::AFile(se) => {
                    if !extensions_filter.accepts_os(&se.ext_os) {
                        continue;
                    }
                    match files.get_mut(&se.stem_os) {
                        None => {
                            files.insert(se.stem_os, vec![se.ext_os]);
                        }
                        Some(exts) => {
                            exts.push(se.ext_os);
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

enum DirOrFile
{
    ADir(PathBuf),
    AFile(StemAndExt),
}

impl DirOrFile
{
    fn from(f_type: &FileType, entry: &fs::DirEntry) -> Option<DirOrFile>
    {
        if f_type.is_dir() {
            Some(DirOrFile::ADir(entry.path()))
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

struct StemAndExt
{
    stem: String,
    stem_os: OsString,
    ext: String,
    ext_os: OsString,
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
            stem_os: OsString::from(stem),
            ext: from_os_str(ext),
            ext_os: OsString::from(ext),
        }
        )
    }
}
