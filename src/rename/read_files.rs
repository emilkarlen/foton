use super::common::FnInfo;
use crate::common::read_files;
use crate::common::read_files::stem_and_exts_builder::stem_to_exts_map_builder;
pub(crate) use crate::common::read_files::types::Extension;
pub(crate) use crate::common::read_files::types::FileNameStem;
use crate::common::read_files::types::{map_dc_multi, DirContents};
use std::collections::HashMap;
use std::io;
use std::path::PathBuf;

pub fn sorted_file_infos(dirs: Vec<PathBuf>, config: &read_files::ReadConfig) -> io::Result<Vec<(PathBuf, DirContents<Vec<FnInfo>>)>>
{
    let dcs = read_files::read_files_and_dirs_multi(dirs, config, &mut stem_to_exts_map_builder())?;
    Ok(map_dc_multi(&to_fn_info_files, dcs))
}

fn to_fn_info_files(files: HashMap<FileNameStem, Vec<Extension>>) -> Vec<FnInfo>
{
    let mut ret_val: Vec<FnInfo> = files.into_iter().map(|(stem, exts)| FnInfo::from_os_str(&stem, &exts)).collect();
    ret_val.sort_by(FnInfo::cmp);
    ret_val
}
