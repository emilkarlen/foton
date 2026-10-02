use crate::rename::common::Rename;
use std::path::Path;
use crate::common::read_files::DirContents;

pub fn report_rename(dir: &Path, rename: (&Path, &Path))
{
    println!("{}: {} -> {}", dir.display(), rename.0.display(), rename.1.display())
}

pub fn report_renames(dcs: &Vec<DirContents<Vec<Rename>>>)
{
    dcs.iter().for_each(report_renames_dc);
}

fn report_renames_dc(dc: &DirContents<Vec<Rename>>)
{
    for rename in dc.files.iter() {
        for (old_fn, new_fn) in rename.renames().iter() {
            report_rename(&dc.dir.path, (old_fn, new_fn));
        }
    }
    for sub_dir in dc.sub_dirs.iter() {
        report_renames_dc(&sub_dir);
    }
}
