use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct Rename
{
    pub new_stem: String,
    pub old_stem: String,
    pub extensions: Vec<String>,
}

impl Rename
{
    pub fn renames(&self) -> Vec<(Box<Path>, Box<Path>)>
    {
        let mut ret_val = Vec::with_capacity(self.extensions.len());
        for extension in &self.extensions {
            let mut old_file_name = PathBuf::from(&self.old_stem);
            old_file_name.set_extension(extension);
            let old_file_name = old_file_name.as_path();
            let mut new_file_name = PathBuf::from(&self.new_stem);
            new_file_name.set_extension(extension);
            let new_file_name = new_file_name.as_path();
            ret_val.push((Box::from(old_file_name), Box::from(new_file_name)));
        }
        ret_val
    }
}

pub struct FnInfo
{
    pub stem: String,
    pub extensions: Vec<String>,
}

impl FnInfo
{
    pub fn from(x: (String, Vec<String>)) -> FnInfo
    {
        FnInfo {
            stem: x.0,
            extensions: x.1,
        }
    }
}
