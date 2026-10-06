use crate::common::read_files::types::DirContents;
use crate::rename::common::Rename;
use crate::rename::report::report_rename;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

const TMP_DIR_NAME: &str = concat!(env!("CARGO_BIN_NAME"), "-enum-names-tmp-dir");

pub fn execute(renames: &Vec<(PathBuf, DirContents<Vec<Rename>>)>) -> io::Result<()>
{
    renames.iter().try_for_each(|(dir, dc)| {
        execute_dc((dir, dc))
    })
}
fn execute_dc(renames: (&PathBuf, &DirContents<Vec<Rename>>)) -> io::Result<()>
{
    rename_in_current_dir(renames)?;
    renames.1.sub_dirs.iter().map(|(pwn, dc)| execute_dc((&pwn.path, dc))).collect()
}

fn rename_in_current_dir(renames: (&PathBuf, &DirContents<Vec<Rename>>)) -> io::Result<()>
{
    let tmp_dir = renames.0.join(TMP_DIR_NAME);
    let tmp_dir = tmp_dir.as_path();
    fs::create_dir(tmp_dir)?;
    move_files_to_tmp_dir(tmp_dir, renames)?;
    rename_by_moving_from_tmp_dir(tmp_dir, renames)?;
    fs::remove_dir(tmp_dir)?;
    Ok(())
}

fn move_files_to_tmp_dir(tmp_dir: &Path, d_dc: (&PathBuf, &DirContents<Vec<Rename>>)) -> io::Result<()>
{
    for rename in d_dc.1.files.iter() {
        for (old_fn, _) in rename.renames().iter() {
            fs::rename(d_dc.0.join(old_fn), tmp_dir.join(old_fn))?;
        }
    }
    Ok(())
}

fn rename_by_moving_from_tmp_dir(tmp_dir: &Path, d_dc: (&PathBuf, &DirContents<Vec<Rename>>)) -> io::Result<()>
{
    for rename in d_dc.1.files.iter() {
        for (old_fn, new_fn) in rename.renames().iter() {
            fs::rename(tmp_dir.join(old_fn), d_dc.0.join(new_fn))?;
            report_rename(&d_dc.0, (old_fn, new_fn));
        }
    }
    Ok(())
}
