use std::path::PathBuf;

pub struct DirContents<FT>
{
    pub dir: PathBuf,
    pub sub_dirs: Vec<DirContents<FT>>,
    pub files: Vec<FT>,
}