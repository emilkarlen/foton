use std::path::PathBuf;
use crate::common::read_files::types::DirContents;
use crate::file_exts::config::ReportConfig;
use crate::file_exts::types::ExtToCount;

mod single_node;
pub mod global;
pub mod per_node;

pub fn global(config: &ReportConfig, collection: &mut ExtToCount)
{
    let data = single_node::report_data(config, collection);
    single_node::print_report(0, config, &data);
}

pub fn per_node(depth_gt_0: usize, config: &ReportConfig, dirs: Vec<(PathBuf, DirContents<ExtToCount>)>)
{
    per_node::per_node(depth_gt_0, config, dirs);
}
