use crate::common;
use crate::common::read_files::types::DirContents;
use crate::file_exts::files_builder;
use crate::file_exts::files_builder_acc;
use crate::file_exts::types::ExtToCount;
use std::io;
use std::path::PathBuf;

pub fn single_global_count(dirs: Vec<PathBuf>, config: &common::read_files::ReadConfig) -> io::Result<ExtToCount>
{
    let mut factory = files_builder_acc::ExtsCountsFactory::new();
    common::read_files::read_files_and_dirs_multi(dirs, config, &mut factory)?;
    Ok(factory.extensions.into_inner())
}

pub fn counter_per_dir(dirs: Vec<PathBuf>, config: &common::read_files::ReadConfig) -> io::Result<Vec<(PathBuf, DirContents<ExtToCount>)>>
{
    let mut factory = files_builder::new_factory();
    common::read_files::read_files_and_dirs_multi(dirs, config, &mut factory)
}
