use std::path::PathBuf;
use std::io;

use super::err::*;

pub fn format_skip(file_name: &String, cause: &str) -> String
{
    format!("{}: {}\n", file_name, cause)
}

pub fn fs_err(cause: &PathFsError, dst: &PathBuf) -> String
{
    match cause {
        PathFsError::Src(err) => fs_src_err(err),
        PathFsError::Dst(err) => fs_dst_err(err, dst),
        PathFsError::Rename(io_err) => fs_rename_err(io_err, dst),
    }
}

pub fn src_str_err(cause: &PathStrError) -> &'static str
{
    match cause {
        PathStrError::MissingFileName =>
            "missing file name",
        PathStrError::MissingExtension =>
            "missing extension",
        PathStrError::NonUnicodeExtension =>
            "non-Unicode extension",
        PathStrError::LowercaseExtension =>
            "extension is lowercase",
    }
}

fn fs_src_err(cause: &SrcPathFsError) -> String
{
    match cause {
        SrcPathFsError::Access(io_error) => io_error.to_string(),
        SrcPathFsError::NotARegularFile => String::from("not a regular file"),
    }
}

fn fs_dst_err(cause: &DstPathFsError, dst: &PathBuf) -> String
{
    match cause {
        DstPathFsError::ExistenceCheck(io_error) => io_error.to_string(),
        DstPathFsError::Exists => format!("Exists: {}", dst.to_string_lossy()),
    }
}
    
fn fs_rename_err(cause: &io::Error, _dst: &PathBuf) -> String
{
    cause.to_string()
}
