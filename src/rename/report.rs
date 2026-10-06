use crate::common::read_files::types::DirContents;
use crate::rename::common::Rename;
use std::path::{Path, PathBuf};

pub fn report_rename(dir: &Path, rename: (&Path, &Path))
{
    println!("{}: {} -> {}", dir.display(), rename.0.display(), rename.1.display())
}

pub fn report_renames(dcs: &Vec<(PathBuf, DirContents<Vec<Rename>>)>)
{
    dcs.iter().for_each(|(p, dc)| report_renames_dc((p, dc)));
}

fn report_renames_dc(dir_dc: (&PathBuf, &DirContents<Vec<Rename>>))
{
    // dbg!("report_renames_dc");
    // dbg!(&dc);
    for rename in dir_dc.1.files.iter() {
        for (old_fn, new_fn) in rename.renames().iter() {
            report_rename(&dir_dc.0, (old_fn, new_fn));
        }
    }
    for (pwn, dc) in dir_dc.1.sub_dirs.iter() {
        report_renames_dc((&pwn.path, dc));
    }
}
