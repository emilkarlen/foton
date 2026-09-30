use crate::rename::common::Rename;
use crate::rename::report::report_rename;
use crate::common::read_files;
use std::fs;
use std::io;
use std::path::Path;

const TMP_DIR_NAME: &str = concat!(env!("CARGO_BIN_NAME"), "-enum-names-tmp-dir");

pub fn execute(renames: &read_files::DirContents<Vec<Rename>>) -> io::Result<u8>
{
    rename_in_current_dir(renames)?;
    for sub_dir in renames.sub_dirs.iter() {
        execute(sub_dir)?;
    }
    Ok(0)
}

fn rename_in_current_dir(renames: &read_files::DirContents<Vec<Rename>>) -> io::Result<u8>
{
    let tmp_dir = renames.dir.path.join(TMP_DIR_NAME);
    let tmp_dir = tmp_dir.as_path();
    fs::create_dir(tmp_dir)?;
    move_files_to_tmp_dir(tmp_dir, &renames)?;
    rename_by_moving_from_tmp_dir(tmp_dir, &renames)?;
    fs::remove_dir(tmp_dir)?;
    Ok(0)
}

fn move_files_to_tmp_dir(tmp_dir: &Path, rid: &read_files::DirContents<Vec<Rename>>) -> io::Result<()>
{
    for rename in rid.files.iter() {
        for (old_fn, _) in rename.renames().iter() {
            fs::rename(rid.dir.path.join(old_fn), tmp_dir.join(old_fn))?;
        }
    }
    Ok(())
}

fn rename_by_moving_from_tmp_dir(tmp_dir: &Path, rid: &read_files::DirContents<Vec<Rename>>) -> io::Result<()>
{
    for rename in rid.files.iter() {
        for (old_fn, new_fn) in rename.renames().iter() {
            fs::rename(tmp_dir.join(old_fn), rid.dir.path.join(new_fn))?;
            report_rename(&rid.dir.path, (old_fn, new_fn));
        }
    }
    Ok(())
}
