use crate::common::ext_filter::ExtensionsFilter;
use crate::utils::from_os_str;
use std::collections::HashMap;
use std::ffi::OsString;
use std::fs;
use std::fs::FileType;
use std::io;
use std::path::{Path, PathBuf};


pub struct PathWithName
{
    pub path: PathBuf,
    pub name: OsString,
}

impl PathWithName
{
    pub fn from(path: PathBuf) -> Option<PathWithName>
    {
        let name = path.file_name()?;
        let nc = OsString::from(name);
        Some(PathWithName { path, name: nc})
    }
}

pub struct DirContents<FILES>
{
    pub dir: PathWithName,
    pub sub_dirs: Vec<DirContents<FILES>>,
    pub files: FILES,
}
pub type FileNameStem = OsString;
pub type  Extension = OsString;

pub type FileNameStems = HashMap<FileNameStem, Vec<Extension>>;

pub type DirPath = PathBuf;

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
    pub include_hidden_sub_dirs: bool,
    pub extensions_filter: Box<dyn ExtensionsFilter>,
}

pub fn group_by_file_name_stem(dir: PathWithName, config: &ReadConfig) -> io::Result<DirContents<HashMap<FileNameStem, Vec<Extension>>>>
{
    let (sub_dir_names, files) = read_files_non_rec(&dir.path, config)?;
    let mut sub_dir_names = sub_dir_names;
    let mut sub_dirs = Vec::with_capacity(sub_dir_names.len());
    if config.recursive {
        for sub_dir_path in sub_dir_names.drain(..).rev() {
            let sub_dir_contents = group_by_file_name_stem(sub_dir_path, config)?;
            sub_dirs.push(sub_dir_contents);
        }
    }
    Ok(DirContents{dir, sub_dirs, files, })
}

fn read_files_non_rec(dir: &Path, config: &ReadConfig) -> io::Result<(Vec<PathWithName>, HashMap<FileNameStem, Vec<Extension>>)>
{
    let mut sub_dirs: Vec<PathWithName> = Vec::new();
    let mut files: HashMap<OsString, Vec<OsString>> = HashMap::new();
    let entries = dir.read_dir()?;
    for mb_entry in entries {
        let entry = mb_entry?;
        let f_type = entry.file_type()?;
        let mb_dof = DirOrFile::from(&f_type, &entry);
        if let Some(dof) = mb_dof {
            match dof {
                DirOrFile::ADir(pwn) => {
                        if config.include_hidden_sub_dirs || !crate::common::fs::is_hidden(&pwn.name) {
                            sub_dirs.push(pwn)
                        }
                }
                DirOrFile::AFile(se) => {
                    if !config.extensions_filter.accepts_os(&se.ext_os) {
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
    ADir(PathWithName),
    AFile(StemAndExt),
}

impl DirOrFile
{
    fn from(f_type: &FileType, entry: &fs::DirEntry) -> Option<DirOrFile>
    {
        if f_type.is_dir() {
            let pwn = PathWithName::from(entry.path())?;
            Some(DirOrFile::ADir(pwn))
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
