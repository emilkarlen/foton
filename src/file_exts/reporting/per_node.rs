use crate::common::read_files::types::DirContents;
use crate::file_exts::config::ReportConfig;
use crate::file_exts::reporting::single_node;
use crate::file_exts::types::ExtToCount;
use std::collections::HashMap;
use std::path::PathBuf;


pub fn per_node(depth_gt_0: usize, config: &ReportConfig, dirs: Vec<(PathBuf, DirContents<ExtToCount>)>)
{
    let summarized: Vec<(PathBuf, DirContents<ExtToCount>)> = dirs.into_iter()
        .map(|(dir, dc)| (dir, sum_sub_dirs_at_depth(0, depth_gt_0, dc)))
        .collect();
    report_roots(config, summarized);
}

pub fn sum_sub_dirs_at_depth(curr_depth: usize, max_depth: usize, root_dc: DirContents<ExtToCount>) -> DirContents<ExtToCount>
{
    if curr_depth < max_depth {
        let DirContents { sub_dirs, files } = root_dc;
        DirContents {
            files,
            sub_dirs: sub_dirs.into_iter()
                .map(|(dir, dc)| (dir, sum_sub_dirs_at_depth(curr_depth+1, max_depth, dc)))
                .collect(),
        }
    } else {
        let mut accumulator: ExtToCount = HashMap::new();
        sum_ext_occurrences(root_dc, &mut accumulator);
        DirContents {
            files: accumulator,
            sub_dirs: Vec::new(),
        }
    }
}

fn sum_ext_occurrences(dc: DirContents<ExtToCount>, accumulator: &mut ExtToCount)
{
    let DirContents { sub_dirs, mut files } = dc;
    for (ext, num_occurs) in files.drain() {
        accumulator.entry(ext).and_modify(|n| *n += num_occurs).or_insert(num_occurs);
    }
    sub_dirs.into_iter().for_each(|p_dc|  sum_ext_occurrences(p_dc.1, accumulator) );
}

fn report_roots(config: &ReportConfig, dcs: Vec<(PathBuf, DirContents<ExtToCount>)>)
{
    dcs.into_iter().for_each(|(dir, dc)| report_dc(1, config, dir, dc));
}

fn report_dc(depth: usize, config: &ReportConfig, dir: PathBuf, dc: DirContents<ExtToCount>)
{
    if !dc.files.is_empty() {
        print_report(depth, config, &dir, dc.files);
    }
    dc.sub_dirs.into_iter().for_each(|(pwn, dc)| report_dc(depth+1, config, pwn.path, dc));
}

fn print_report(depth: usize, config: &ReportConfig, dir: &PathBuf, exts: ExtToCount)
{
    println!("{}:", dir.to_string_lossy());
    single_node::report(depth, config, exts);
}