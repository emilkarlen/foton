use std::ffi::OsStr;

pub struct ReadConfig
{
    pub recursive: bool,
    pub include_hidden_sub_dirs: bool,
    pub split_stem_and_ext: fn(&OsStr) -> (&OsStr, Option<&OsStr>)

}

pub struct ReportConfig
{
    pub num_files: bool,
    pub tot_num_files: bool,
    pub sort_on_num_ext: bool,
    pub sort_reverse: bool,
}
