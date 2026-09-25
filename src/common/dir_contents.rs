use std::path::PathBuf;

pub struct DirContents<FILES>
{
    pub dir: PathBuf,
    pub sub_dirs: Vec<DirContents<FILES>>,
    pub files: FILES,
}