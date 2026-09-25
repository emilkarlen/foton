use crate::common::dir_contents::DirContents;
use crate::enum_names::common::Rename;
use crate::enum_names::report::report_rename;
use std::fs;
use std::io;
use std::path::Path;

const TMP_DIR_NAME: &str = concat!(env!("CARGO_BIN_NAME"), "-enum-names-tmp-dir");

pub fn execute(renames: &DirContents<Rename>) -> io::Result<u8>
{
    rename_in_current_dir(renames)?;
    for sub_dir in renames.sub_dirs.iter() {
        execute(sub_dir)?;
    }
    Ok(0)
}

fn rename_in_current_dir(renames: &DirContents<Rename>) -> io::Result<u8>
{
    let tmp_dir = renames.dir.join(TMP_DIR_NAME);
    let tmp_dir = tmp_dir.as_path();
    fs::create_dir(tmp_dir)?;
    move_files_to_tmp_dir(tmp_dir, &renames)?;
    rename_by_moving_from_tmp_dir(tmp_dir, &renames)?;
    fs::remove_dir(tmp_dir)?;
    Ok(0)
}

fn move_files_to_tmp_dir(tmp_dir: &Path, rid: &DirContents<Rename>) -> io::Result<()>
{
    for rename in rid.files.iter() {
        for (old_fn, _) in rename.renames().iter() {
            fs::rename(rid.dir.join(old_fn), tmp_dir.join(old_fn))?;
            // println!("{} -> {}", rid.dir.join(old_fn).display(), tmp_dir.join(old_fn).display());
        }
    }
    Ok(())
}

fn rename_by_moving_from_tmp_dir(tmp_dir: &Path, rid: &DirContents<Rename>) -> io::Result<()>
{
    for rename in rid.files.iter() {
        for (old_fn, new_fn) in rename.renames().iter() {
            fs::rename(tmp_dir.join(old_fn), rid.dir.join(new_fn))?;
            // println!("{} -> {}", tmp_dir.join(old_fn).display(), rid.dir.join(new_fn).display());
            report_rename(&rid.dir, (old_fn, new_fn));
        }
    }
    Ok(())
}