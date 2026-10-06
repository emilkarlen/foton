use crate::common;
use crate::common::read_files::types::DirContents;
use crate::file_exts::files_builder;
use crate::file_exts::files_builder::ExtToCount;
use std::collections::HashMap;
use std::ffi::OsString;
use std::io;
use std::path::PathBuf;

pub fn execute(dirs: Vec<PathBuf>, config: &common::read_files::ReadConfig) -> io::Result<HashMap<OsString, usize>>
{
    let dirs_contents = common::read_files::read_files_and_dirs_multi(dirs, config, &mut files_builder::new_factory())?;
    let mut accumulator: ExtToCount = HashMap::new();
    dirs_contents.into_iter().for_each(|(_, dc)| sum_ext_occurrences(dc, &mut accumulator));
    Ok(accumulator)
}

fn sum_ext_occurrences(dc: DirContents<ExtToCount>, accumulator: &mut ExtToCount)
{
    let DirContents { sub_dirs, mut files } = dc;
    for (ext, num_occurs) in files.drain() {
        accumulator.entry(ext).and_modify(|n| *n += num_occurs).or_insert(num_occurs);
    }
    sub_dirs.into_iter().for_each(|p_dc|  sum_ext_occurrences(p_dc.1, accumulator) );
}
