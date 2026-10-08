use std::iter;
use crate::file_exts::config::ReportConfig;
use crate::file_exts::types::ExtToCount;
use crate::utils;

pub struct Data
{
    extensions: Vec<(String, usize)>,
    tot_num_files: usize,
    max_num_files_per_ext: usize,
    max_ext_len: usize,
}

const INDENT_STR: &str = "  ";

pub fn report(indent: usize, config: &ReportConfig, mut collection: ExtToCount)
{
    print_report(indent, config, &report_data(config, &mut collection));
}
pub fn report_data(config: &ReportConfig, collection: &mut ExtToCount) -> Data
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
        max_num_files_per_ext: max_num_files,
        max_ext_len,
    }
}

pub fn print_report(indent: usize, config: &ReportConfig, data: &Data)
{
    let indent_str: String = iter::repeat_n(INDENT_STR, indent).collect();

    let num_formatter = utils::FixedWidthFormatter::new_for_num(data.max_num_files_per_ext, 0);
    let ext_formatter = utils::FixedWidthFormatter::new(data.max_ext_len);
    for (ext, num) in data.extensions.iter() {
        if config.num_files {
            println!("{indent_str}{} {}", ext_formatter.str(&ext), num_formatter.num_spc(*num));
        } else {
            println!("{indent_str}{ext}");
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
        println!("{indent_str}{}", formatter.num_spc(data.tot_num_files));
    }
}

fn sort_res(config: &ReportConfig, extensions: &mut Vec<(String, usize)>)
{
    if config.sort_on_num_ext {
        extensions.sort_by(cmp_on_num_ext);
    } else {
        extensions.sort();
    }
    if config.sort_reverse {
        extensions.reverse();
    }
}

fn cmp_on_num_ext((xe, xn): &(String, usize), (ye,yn): &(String, usize)) -> core::cmp::Ordering
{
    (xn, xe).cmp(&(yn, ye))
}