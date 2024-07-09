use std::io;
use std::path::PathBuf;

use super::err::*;

pub struct Rename
{
    pub src: PathBuf,
    pub dst: PathBuf,
}

pub struct Renamer
{
    action: fn(&Rename) -> Result<(), io::Error>,
}

impl Renamer
{

    pub fn resolve(execute: bool) -> Renamer
    {
        if execute {
            Renamer {
                action: Renamer::do_rename,
            }
        } else {
            Renamer {
                action: Renamer::do_nothing,
            }
        }
    }

    pub fn apply(&self, rename: &Rename) -> Result<(), PathFsError>
    {
        self.check(rename)?;
        (self.action)(rename).map_err(|e| PathFsError::Rename(e))
    }

    fn check(&self, rename: &Rename) -> Result<(), PathFsError>
    {
        self.check_src(&rename.src).map_err(|se| PathFsError::Src(se))?;
        self.check_dst(&rename.dst).map_err(|se| PathFsError::Dst(se))
    }

    fn check_src(&self, src: &PathBuf) -> Result<(), SrcPathFsError>
    {
        let meta_data_result = src.metadata();
        match meta_data_result {
            Err(io_error) => Err(SrcPathFsError::Access(io_error)),
            Ok(meta_data) => {
                if meta_data.is_file() { 
                    Ok(())
                }
                else {
                    Err(SrcPathFsError::NotARegularFile) 
                }
            }
        }
    }

    fn check_dst(&self, dst: &PathBuf) -> Result<(), DstPathFsError>
    {
        let exists_r = dst.try_exists();
        match exists_r {
            Err(io_error) => {
                Err(DstPathFsError::ExistenceCheck(io_error)
            ) },
            Ok(dst_exists) => {
                if dst_exists { 
                    Err(DstPathFsError::Exists) 
                }
                else {
                    Ok(())
                }
            }
        }
    }

    fn do_nothing(_rename: &Rename) -> Result<(), io::Error>
    {
        Ok(())
    }

    fn do_rename(rename: &Rename) -> Result<(), io::Error>
    {
        std::fs::rename(rename.src.as_path(), rename.dst.as_path())
    }
}
