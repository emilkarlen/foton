use crate::utils;
use std::collections::HashMap;
use std::ffi::OsString;
use crate::file_exts::config::ReportConfig;

pub fn execute(config: &ReportConfig, collection: &mut HashMap<OsString, usize>) {
    let data = report_data(config, collection);
    report(config, &data);
}

struct Data
{
    extensions: Vec<(String, usize)>,
    tot_num_files: usize,
    max_num_files: usize,
    max_ext_len: usize,
}

fn report_data(config: &ReportConfig, collection: &mut HashMap<OsString, usize>) -> Data
{
    let mut extensions: Vec<(String, usize)> = Vec::new();
    let mut tot_num_files: usize = 0;
    let mut max_num_files = 0;
    let mut max_ext_len = 0;
    for (ext, num) in collection.drain() {
        extensions.push((utils::from_os_str(&ext), num));
        tot_num_files += num;
        if num > max_num_files {
            max_num_files = num;
        }
        if ext.len() > max_ext_len {
            max_ext_len = ext.len();
        }
    }
    sort_res(config, &mut extensions);

    Data {
        extensions,
        tot_num_files,
        max_num_files,
        max_ext_len,
    }
}

fn report(config: &ReportConfig, data: &Data)
{
    let num_formatter = utils::FixedWidthFormatter::new_for_num(data.max_num_files);
    let ext_formatter = utils::FixedWidthFormatter::new(data.max_ext_len);
    for (ext, num) in data.extensions.iter() {
        if config.num_files {
            println!("{} {}", ext_formatter.str(&ext), num_formatter.num_spc(*num));
        } else {
            println!("{ext}");
        }
    }
    if config.tot_num_files {
        let formatter = if config.num_files {
            let width = num_formatter.width + ext_formatter.width + 1;
            utils::FixedWidthFormatter::new(width)
        } else {
            print!(" ");
            utils::FixedWidthFormatter::new(data.max_ext_len)
        };
        println!("{}", formatter.num_spc(data.tot_num_files));
    }
}

fn sort_res(config: &ReportConfig, extensions: &mut Vec<(String, usize)>)
{
    if config.sort_on_num_ext {
        extensions.sort_by(cmd_on_num_ext);
    } else {
        extensions.sort();
    }
    if config.sort_reverse {
        extensions.reverse();
    }
}

fn cmd_on_num_ext((xe, xn): &(String, usize), (ye,yn): &(String, usize)) -> core::cmp::Ordering
{
    (xn, xe).cmp(&(yn, ye))
}
