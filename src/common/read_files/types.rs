use std::ffi::{OsStr, OsString};
use std::fs;
use std::path::PathBuf;

pub type FileNameStem = OsString;

pub type  Extension = OsString;

pub type StemExtSplitter = fn(&OsStr) -> (&OsStr, Option<&OsStr>);

pub struct StemAndExt
{
    pub stem: OsString,
    pub ext: OsString,
}

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
impl StemAndExt
{
    pub fn from(entry: &fs::DirEntry, splitter: StemExtSplitter) -> Option<StemAndExt>
    {
        let file_name = entry.file_name();
        let (stem, mb_ext) = splitter(&file_name);
        if let Some(ext) = mb_ext {
            Some(StemAndExt {
                stem: OsString::from(stem),
                ext: OsString::from(ext),
            })
        }
        else {
            None
                 }
        }
}

#[derive(Debug)]
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
        Some(PathWithName { path, name: nc })
    }
}

#[derive(Debug)]
pub struct DirContents<FILES>
{
    pub sub_dirs: Vec<(PathWithName, DirContents<FILES>)>,
    pub files: FILES,
}

pub fn map_dc<T1, T2, F>(f: &F, dc: DirContents<T1>) -> DirContents<T2>
where
    F: Fn(T1) -> T2,
{
    let DirContents { sub_dirs, files} = dc;
    let sub_dirs2: Vec<(PathWithName, DirContents<T2>)> = sub_dirs.into_iter().map(|(n, dc)| (n, map_dc(f, dc))).collect();
    DirContents {
        sub_dirs: sub_dirs2,
        files: f(files),
    }
}

// Vec<(PathBuf, DirContents2<Vec<FnInfo>>)>
pub fn map_dc_multi<T1, T2, F>(f: &F, dcs: Vec<(PathBuf, DirContents<T1>)>) -> Vec<(PathBuf, DirContents<T2>)>
where
    F: Fn(T1) -> T2,
{
    dcs.into_iter().map(|(n, dc)| (n, map_dc(f, dc))).collect()
}
