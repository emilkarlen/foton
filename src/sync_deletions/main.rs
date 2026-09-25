use crate::common::read_files;
use crate::common::read_files::{DirContents, Extension, FileNameStem, PathWithName, ReadConfig};
use std::collections::HashMap;
use std::ffi::OsString;
use std::io;
use crate::common::ext_filter::any;

pub fn with_valid_args(execute: bool, dir_src: PathWithName, dir_dst: PathWithName, read_config: &ReadConfig) -> io::Result<()>
{
    let src_files_config = ReadConfig {
        extensions_filter: Box::new(any()),
        ..*read_config
    };
    let src_files = read_files::group_by_file_name_stem(dir_src, &src_files_config)?;
    let dst_files = read_files::group_by_file_name_stem(dir_dst, &read_config)?;
    let dst_deleted = filter_not_in_src(dst_files, &src_files);
    process(dst_deleted);
    Ok(())
}

fn report(dst_deleted: MyDirContents)
{
    report_files(&dst_deleted);
    let mut sub_dirs = dst_deleted.sub_dirs;
    sub_dirs.sort_by(|x,y| x.dir.name.cmp(&y.dir.name));
}
fn report_files(dst_deleted: &MyDirContents)
{
    println!("{} {}", dst_deleted.dir.path.display(), dst_deleted.dir.name.display());
    for (stem, _) in dst_deleted.files.iter() {
        println!("  {}", stem.display());
    }
}
fn process_files(dst_deleted: &MyDirContents)
{
    let dir = &dst_deleted.dir.path;
    for (stem, _) in dst_deleted.files.iter() {
        let p = dir.join(stem);
        println!("rm '{}'.*", p.display());
    }
}

fn process_files2(dir: &PathWithName, files: &Vec<(FileNameStem, Vec<Extension>)>)
{
    for (stem, _) in files.iter() {
        let p = dir.path.join(stem);
        println!("rm '{}'.*", p.display());
    }
}

fn process(dst_deleted: MyDirContents)
{
    // process_files(&dst_deleted);
    let dir = dst_deleted.dir;
    let mut files_map = dst_deleted.files;
    let mut files = Vec::with_capacity(files_map.len());
    for (stem, exts) in files_map.drain() {
        files.push((stem, exts));
    }
    files.sort_by(|x, y| x.0.cmp(&y.0));
    process_files2(&dir, &files);
    let mut sub_dirs = dst_deleted.sub_dirs;
    sub_dirs.sort_by(|x,y| x.dir.name.cmp(&y.dir.name));
    for sub_dir in sub_dirs.drain(..) {
        process(sub_dir);
    }
}


type MyFileContents = HashMap<FileNameStem, Vec<Extension>>;
type MyDirContents = DirContents<MyFileContents>;
fn filter_not_in_src(dst: MyDirContents, src: &MyDirContents) -> MyDirContents

{
    let mut sub_dirs: Vec<MyDirContents> = Vec::with_capacity(dst.sub_dirs.len());
    let mut files: MyFileContents = HashMap::with_capacity(dst.files.len());
    let dst_dir = dst.dir;
    let mut dst_files = dst.files;
    let mut dst_sub_dirs = dst.sub_dirs;
    for (stem, exts) in dst_files.drain() {
        if !src.files.contains_key(&stem) {
            files.insert(stem, exts);
        }
    }
    for dst_sub_dir in dst_sub_dirs.drain(..) {
        if let Some(src_sub_dir) = find_sub_dir_by_file_name(&dst_sub_dir.dir.name, &src.sub_dirs) {
            let filtered_dir  = filter_not_in_src(dst_sub_dir, src_sub_dir);
            sub_dirs.push(filtered_dir);
        }
        else {
            sub_dirs.push(dst_sub_dir);
        }
    }
    DirContents {
        dir: dst_dir,
        sub_dirs,
        files,
    }
}

fn find_sub_dir_by_file_name<'a, 'b, T>(name: &'a OsString, sub_dirs: &'b Vec<DirContents<T>>) -> Option<&'b DirContents<T>>
{
    sub_dirs.iter().find(|x| x.dir.name == *name)
}
