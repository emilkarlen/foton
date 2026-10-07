use crate::common;
use crate::common::read_files::types::DirContents;
use crate::file_exts::files_builder_acc;
use crate::file_exts::types::ExtToCount;
use std::io;
use std::path::PathBuf;

pub fn execute(dirs: Vec<PathBuf>, config: &common::read_files::ReadConfig) -> io::Result<ExtToCount>
{
    let mut factory = files_builder_acc::ExtsCountsFactory::new();
    common::read_files::read_files_and_dirs_multi(dirs, config, &mut factory)?;
    Ok(factory.extensions.into_inner())
}

fn sum_ext_occurrences(dc: DirContents<ExtToCount>, accumulator: &mut ExtToCount)
{
    let DirContents { sub_dirs, mut files } = dc;
    for (ext, num_occurs) in files.drain() {
        accumulator.entry(ext).and_modify(|n| *n += num_occurs).or_insert(num_occurs);
    }
    sub_dirs.into_iter().for_each(|p_dc|  sum_ext_occurrences(p_dc.1, accumulator) );
}
