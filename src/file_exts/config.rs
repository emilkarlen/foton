use std::ffi::OsString;
use std::path::Path;

pub struct ReadConfig
{
    pub recursive: bool,
    pub include_hidden_sub_dirs: bool,
    pub get_ext: fn(&Path) -> Option<OsString>,
}

pub struct ReportConfig
{
    pub num_files: bool,
    pub tot_num_files: bool,
    pub sort_on_num_ext: bool,
    pub sort_reverse: bool,
}
