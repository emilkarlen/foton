use crate::common::read_files::DirContents;
use crate::rename::common::Rename;
use crate::rename::report::report_rename;
use std::fs;
use std::io;
use std::path::Path;

const TMP_DIR_NAME: &str = concat!(env!("CARGO_BIN_NAME"), "-enum-names-tmp-dir");

pub fn execute(renames: &Vec<DirContents<Vec<Rename>>>) -> io::Result<()>
{
    renames.iter().try_for_each(|dc| {
        execute_dc(dc)
    })
}
fn execute_dc(renames: &DirContents<Vec<Rename>>) -> io::Result<()>
{
    rename_in_current_dir(renames)?;
    renames.sub_dirs.iter().map(execute_dc).collect()
}

fn rename_in_current_dir(renames: &DirContents<Vec<Rename>>) -> io::Result<()>
{
    let tmp_dir = renames.dir.path.join(TMP_DIR_NAME);
    let tmp_dir = tmp_dir.as_path();
    fs::create_dir(tmp_dir)?;
    move_files_to_tmp_dir(tmp_dir, &renames)?;
    rename_by_moving_from_tmp_dir(tmp_dir, &renames)?;
    fs::remove_dir(tmp_dir)?;
    Ok(())
}

fn move_files_to_tmp_dir(tmp_dir: &Path, dc: &DirContents<Vec<Rename>>) -> io::Result<()>
{
    for rename in dc.files.iter() {
        for (old_fn, _) in rename.renames().iter() {
            fs::rename(dc.dir.path.join(old_fn), tmp_dir.join(old_fn))?;
        }
    }
    Ok(())
}

fn rename_by_moving_from_tmp_dir(tmp_dir: &Path, dc: &DirContents<Vec<Rename>>) -> io::Result<()>
{
    for rename in dc.files.iter() {
        for (old_fn, new_fn) in rename.renames().iter() {
            fs::rename(tmp_dir.join(old_fn), dc.dir.path.join(new_fn))?;
            report_rename(&dc.dir.path, (old_fn, new_fn));
        }
    }
    Ok(())
}
