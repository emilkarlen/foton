use crate::common::ext_filter::any;
use crate::common::read_files;
use crate::common::read_files::{DirContents, Extension, FileNameStem, PathWithName, ReadConfig};
use crate::sync_deletions::config::ProcessConfig;
use std::collections::HashMap;
use std::ffi::OsString;
use std::io;
use std::path::{Path, PathBuf};

pub fn with_valid_args(process_config: &ProcessConfig, dir_src: PathWithName, dir_dst: PathWithName, read_config: &ReadConfig) -> io::Result<()>
{
    let src_files_config = ReadConfig {
        extensions_filter: Box::new(any()),
        ..*read_config
    };
    let src_files = read_files::group_by_file_name_stem(dir_src, &src_files_config)?;
    let dst_files = read_files::group_by_file_name_stem(dir_dst, &read_config)?;
    let dst_deleted = filter_not_in_src(dst_files, &src_files);
    let mut flat = Vec::new();
    flatten(dst_deleted, PathBuf::from(""), &mut flat);
    process(process_config, flat)?;
    Ok(())
}

type ResultForProcessing = Vec<(PathBuf, PathBuf, Vec<(FileNameStem, Vec<Extension>)>)>;

fn flatten(dst_deleted: MyDirContents, dir_under_dst: PathBuf, out: &mut ResultForProcessing)
{
    // process_files(&dst_deleted);
    let dir = dst_deleted.dir;
    let dir_path = dir.path;
    let dir_under_dst_clone = dir_under_dst.clone();
    let mut files_map = dst_deleted.files;
    let mut files = Vec::with_capacity(files_map.len());
    for (stem, exts) in files_map.drain() {
        files.push((stem, exts));
    }
    files.sort_by(|x, y| x.0.cmp(&y.0));
    out.push((dir_path, dir_under_dst, files));
    let mut sub_dirs = dst_deleted.sub_dirs;
    sub_dirs.sort_by(|x,y| x.dir.name.cmp(&y.dir.name));
    for sub_dir in sub_dirs.drain(..) {
        let sub_dir_dud = join(&dir_under_dst_clone, &sub_dir);
        flatten(sub_dir, sub_dir_dud, out);
    }
}

fn join(dir_under_dst: &PathBuf, sub_dir: &MyDirContents) -> PathBuf
{
    let sub_dir = &sub_dir.dir;
    let sub_dir_name = &sub_dir.name;
    dir_under_dst.join(sub_dir_name)
}
fn process(process_config: &ProcessConfig, dirs: ResultForProcessing) -> io::Result<()>
{
    if let Some(move_to_root_dir) = (&process_config).move_to.as_ref() {
        process_move(process_config, move_to_root_dir, dirs)
    } else {
        process_delete(process_config, dirs)
    }
}
fn process_move(process_config: &ProcessConfig, move_to_root_dir: &PathBuf, dirs: ResultForProcessing) -> io::Result<()>
{
    for (dst_dir, dir_under_dst, stem_and_exts)  in dirs.iter() {
        for (stem, exts) in stem_and_exts.iter() {
            let stem_path = PathBuf::from(stem);
            let move_to_sub_dir = move_to_root_dir.join(dir_under_dst);
            let move_to_dir : &Path = if dir_under_dst.is_empty() {
                move_to_root_dir
            } else {
                &move_to_sub_dir
            };
            for ext in exts.iter() {
                let file_name = stem_path.with_added_extension(ext);
                let dst_path = dst_dir.join(&file_name);
                println!("mv '{}' '{}'", dst_path.display(), move_to_dir.display());
                if process_config.execute {
                    if !move_to_dir.exists() {
                        std::fs::create_dir_all(move_to_dir)?;
                    }
                    let mov_path = move_to_dir.join(file_name);
                    std::fs::rename(dst_path, mov_path)?;
                }
            }
        }
    }
    Ok(())
}
fn process_delete(process_config: &ProcessConfig, dirs: ResultForProcessing) -> io::Result<()>
{
    for (dir, _dir_under_dst, stem_and_exts)  in dirs.iter() {
        for (stem, exts) in stem_and_exts.iter() {
            let dst_path_w_stem = dir.join(stem);
            for ext in exts.iter() {
                let dst_path = dst_path_w_stem.with_added_extension(ext);
                println!("rm '{}'", dst_path.display());
                if process_config.execute {
                    std::fs::remove_file(dst_path)?;
                }
            }
        }
    }
    Ok(())
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
