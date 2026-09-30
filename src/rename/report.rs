use crate::common::read_files;
use crate::rename::common::Rename;
use std::path::Path;

pub fn report_rename(dir: &Path, rename: (&Path, &Path))
{
    println!("{}: {} -> {}", dir.display(), rename.0.display(), rename.1.display())
}

pub fn report_renames(rid: &read_files::DirContents<Vec<Rename>>)
{
    for rename in rid.files.iter() {
        for (old_fn, new_fn) in rename.renames().iter() {
            report_rename(&rid.dir.path, (old_fn, new_fn));
        }
    }
    for sub_dir in rid.sub_dirs.iter() {
        report_renames(&sub_dir);
    }
}
