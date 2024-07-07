use std::io;

/// Invalid path string
pub enum PathStrError
{
    MissingFileName,
    MissingExtension,
    NonUnicodeExtension,
    LowercaseExtension,
}


/// Invalid file system files
pub enum PathFsError
{
    Src(SrcPathFsError),
    Dst(DstPathFsError),
    Rename(io::Error),
}

pub enum SrcPathFsError
{
    Access(io::Error),
    NotARegularFile,
}

pub enum DstPathFsError
{
    ExistenceCheck(io::Error),
    Exists,
}
