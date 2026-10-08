use crate::common::read_files::files_builder::{FilesBuilder, FilesBuilderFactory};
use crate::common::read_files::types::{DirContents, PathWithName, StemAndExt, StemExtSplitter};
pub(crate) use config::ReadConfig;
use std::fs::FileType;
use std::path::PathBuf;
use std::{fs, io};

pub mod files_builder;
pub mod types;
pub mod stem_and_exts_builder;
pub mod config;
pub mod config_cli;

pub fn read_files_and_dirs<FILES, FACTORY>(
    dir: &PathBuf,
    config: &ReadConfig,
    files_builder_factory: &mut FACTORY,
) -> io::Result<DirContents<FILES>>
where
    FACTORY: FilesBuilderFactory<FILES>
{
    let (mut sub_dir_names, files) = read_files_non_rec(dir, config, files_builder_factory.new())?;
    let mut sub_dirs = Vec::with_capacity(sub_dir_names.len());
    if config.recursive {
        for sub_dir_path in sub_dir_names.drain(..).rev() {
            let sub_dir_contents = read_files_and_dirs(&sub_dir_path.path, config, files_builder_factory)?;
            sub_dirs.push((sub_dir_path, sub_dir_contents));
        }
    }
    Ok(DirContents {sub_dirs, files, })
}

pub fn read_files_and_dirs_multi<T, FACTORY>(
    dirs: Vec<PathBuf>,
    config: &ReadConfig,
    files_builder_factory: &mut FACTORY,
) -> io::Result<Vec<(PathBuf, DirContents<T>)>>
where
    FACTORY: FilesBuilderFactory<T>
{
    let mut ret_val = Vec::with_capacity(dirs.len());
    for pwn in dirs {
        let x = read_files_and_dirs(&pwn, config, files_builder_factory)?;
        ret_val.push((pwn, x));
    }
    Ok(ret_val)
}

fn read_files_non_rec<T, BUILDER>(
    dir: &PathBuf,
    config: &ReadConfig,
    mut files_builder: Box<BUILDER>,
) -> io::Result<(Vec<PathWithName>, T)>
where
    BUILDER: FilesBuilder<T> + ?Sized
{
    let mut sub_dirs: Vec<PathWithName> = Vec::new();
    let entries = dir.read_dir()?;
    for mb_entry in entries {
        let entry = mb_entry?;
        let f_type = entry.file_type()?;
        let mb_dof = DirOrFile::from(&f_type, &entry, config.split_stem_and_ext);
        if let Some(dof) = mb_dof {
            match dof {
                DirOrFile::ADir(pwn) => {
                        if config.include_hidden_sub_dirs || !crate::common::path::is_hidden(&pwn.name) {
                            sub_dirs.push(pwn)
                        }
                }
                DirOrFile::AFile(se) => {
                    if !config.extensions_filter.accepts_os(&se.ext) {
                        continue;
                    }
                    files_builder.add(se);
                }
            }
        }
    }
    Ok((sub_dirs, files_builder.build()))
}

enum DirOrFile
{
    ADir(PathWithName),
    AFile(StemAndExt),
}

impl DirOrFile
{
    fn from(f_type: &FileType, entry: &fs::DirEntry, splitter: StemExtSplitter) -> Option<DirOrFile>
    {
        let file_name = entry.file_name();
        if f_type.is_dir() {
            let pwn = PathWithName {path: entry.path(), name: file_name};
            Some(DirOrFile::ADir(pwn))
        }
        else if f_type.is_file() {
            let x = StemAndExt::from(entry, splitter)?;
            Some(DirOrFile::AFile(x))
        }
        else {
            None
        }
    }
}
