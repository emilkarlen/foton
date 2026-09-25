use crate::common::dir_contents::DirContents;
use crate::enum_names::common::{FnInfo, Rename};
use crate::enum_names::stem_formatter;
use crate::enum_names::stem_formatter::StemFormatter;

pub fn resolve(dc: &mut DirContents<FnInfo>) -> DirContents<Rename>
{
    let num_stems = num_stems_in(dc);
    let mut sf = stem_formatter::StemFormatter::new(num_stems);
    renames_of(dc, &mut sf)
}

fn renames_of(dc: &mut DirContents<FnInfo>, sf: &mut StemFormatter) -> DirContents<Rename>
{
    let mut sub_dirs = Vec::with_capacity(dc.sub_dirs.len());
    let mut files = Vec::with_capacity(dc.files.len());
    while !dc.files.is_empty() {
        if let Some(fni) = dc.files.pop() {
            let ren = Rename {
                new_stem: sf.format(),
                old_stem: fni.stem,
                extensions: fni.extensions,
            };
            files.push(ren);
        }
    };
    while !dc.sub_dirs.is_empty() {
        if let Some(mut sub_dir) = dc.sub_dirs.pop() {
            sub_dirs.push(renames_of(&mut sub_dir, sf));
        }
    };
    DirContents {
        dir: dc.dir.clone(),
        sub_dirs,
        files,
    }
}

fn num_stems_in(x: &DirContents<FnInfo>) -> usize
{
    let mut ret_val = x.files.len();
    for sub_dir in x.sub_dirs.iter() {
        ret_val += num_stems_in(sub_dir);
    }
    ret_val
}
