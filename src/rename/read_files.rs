use super::common::FnInfo;
use crate::common;
use crate::common::read_files::DirContents;
pub(crate) use crate::common::read_files::{Extension, FileNameStem, PathWithName, ReadConfig};
use std::collections::HashMap;
use std::io;

pub fn rev_sorted_file_infos(dir: PathWithName, config: &ReadConfig) -> io::Result<DirContents<Vec<FnInfo>>>
{
    let gb_stem = common::read_files::group_by_file_name_stem(dir, config)?;
    Ok(to_fn_info_dc(gb_stem))
}

fn to_fn_info_dc(dc: DirContents<HashMap<FileNameStem, Vec<Extension>>>) -> DirContents<Vec<FnInfo>>
{
    let DirContents { dir, mut sub_dirs, mut files } = dc;
    
    let mut sub_dirs1 = Vec::with_capacity(sub_dirs.len());
    for sub_dir in sub_dirs.drain(..) {
        sub_dirs1.push(to_fn_info_dc(sub_dir));
    }
    DirContents {
        dir: dir,
        files: to_fn_info_files(&mut files),
        sub_dirs: sub_dirs1,
    }
}

fn to_fn_info_files(files: &mut HashMap<FileNameStem, Vec<Extension>>) -> Vec<FnInfo>
{
    let mut ret_val = Vec::new();
    for (stem, exts) in files.drain() {
        ret_val.push(FnInfo::from_os_str(&stem, &exts));
    }
    ret_val.sort_by(FnInfo::cmp);
    ret_val
}
