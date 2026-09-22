use std::path::Path;

pub fn report_rename(dir: &Path, rename: (&Path, &Path))
{
    println!("{}: {} -> {}", dir.display(), rename.0.display(), rename.1.display())
}