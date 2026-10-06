use crate::command::{ExeError, UnableToExecuteError};
use crate::common::read_files::types::{DirContents, PathWithName};
use crate::rename::common::{FnInfo, Rename};
use crate::rename::config::NamingConfig;
use crate::rename::renamer;
use crate::rename::renamer::StemGenerator;
use std::collections::HashMap;
use std::path::PathBuf;

const REASON: &str = "Name clashes";

pub fn resolve(dcs: Vec<(PathBuf, DirContents<Vec<FnInfo>>)>, config: &NamingConfig) -> Result<Vec<(PathBuf, DirContents<Vec<Rename>>)>, ExeError>
{
    let mut name_generator = renamer::resolve(&dcs, config);
    let renames = renames_of(dcs, &mut name_generator);
    if name_generator.may_produce_clashes() {
        with_check_for_clashes(renames)
    }
    else {
            Ok(renames)
    }
}

fn with_check_for_clashes(dir_renames: Vec<(PathBuf, DirContents<Vec<Rename>>)>) -> Result<Vec<(PathBuf, DirContents<Vec<Rename>>)>, ExeError>
{
    let mut errs = Vec::new();
    let renames = check_clashes(dir_renames, &mut errs);
    if errs.is_empty() {
        Ok(renames)
    } else {
        let msg = UnableToExecuteError {
            reason: String::from(REASON),
            details: err_msg_details(errs),
        };
        Err(ExeError::UnableToExecute(msg))
    }
}

fn err_msg_details(errors: Vec<(PathBuf, Vec<Rename>)>) -> Vec<String>
{
    let mut ret_val = Vec::new();
    for (path, renames) in errors {
        for rename in renames {
            ret_val.push(err_msg_detail(&path, &rename));
        }
    }
    ret_val
}

fn err_msg_detail(path: &PathBuf, rename: &Rename) -> String
{
    format!("{}: {} -> {}", path.display(), rename.old_stem, rename.new_stem)
}
fn renames_of(dcs: Vec<(PathBuf, DirContents<Vec<FnInfo>>)>, stem_gen: &mut Box<dyn StemGenerator>) -> Vec<(PathBuf, DirContents<Vec<Rename>>)>
{
    dcs.into_iter().map(|(n, dc)| (n, renames_of_dc(dc, stem_gen))).collect()
}
fn renames_of_dc(dc: DirContents<Vec<FnInfo>>, stem_gen: &mut Box<dyn StemGenerator>) -> DirContents<Vec<Rename>>
{
    let DirContents { sub_dirs, mut files} = dc;

    let mut files_2 = Vec::with_capacity(files.len());
    for fni in files.drain(..) {
        let ren = Rename {
            new_stem: stem_gen.next(fni.stem.as_str()),
            old_stem: fni.stem,
            extensions: fni.extensions,
        };
        // dbg!(&ren);
        files_2.push(ren);

    }
    let sd2 = sub_dirs.into_iter().map(|(n, dc)| (n, renames_of_dc(dc, stem_gen))).collect();
    DirContents {
        sub_dirs: sd2,
        files: files_2,
    }
}

fn check_clashes(dcs: Vec<(PathBuf, DirContents<Vec<Rename>>)>, errs: &mut Vec<(PathBuf, Vec<Rename>)>) -> Vec<(PathBuf, DirContents<Vec<Rename>>)>
{
    dcs.into_iter().map(|(dir, dc)| {
        let dc_checked = check_clashes_dc(&dir, dc, errs);
        (dir, dc_checked)
    }).collect()
}

fn check_clashes_dc(dir: &PathBuf, dc: DirContents<Vec<Rename>>, errs: &mut Vec<(PathBuf, Vec<Rename>)>) -> DirContents<Vec<Rename>>
{
    fn process_sub_dirs(sub_dirs: Vec<(PathWithName, DirContents<Vec<Rename>>)>, errs: &mut Vec<(PathBuf, Vec<Rename>)>) -> Vec<(PathWithName, DirContents<Vec<Rename>>)>
    {
        sub_dirs.into_iter().map(|(pwn, dc)| {
            let dc_checked = check_clashes_dc(&pwn.path, dc, errs);
            (pwn, dc_checked)
        }).collect()
    }

    let DirContents { sub_dirs, mut files } = dc;
    let (non_clashes, clashes) = check_files(&mut files);
    if !clashes.is_empty() {
        errs.push((dir.clone(), clashes));
    }
    DirContents {
        files: non_clashes,
        sub_dirs: process_sub_dirs(sub_dirs, errs),
    }}

fn check_files(files: &mut Vec<Rename>) -> (Vec<Rename>, Vec<Rename>)
{
    let mut base_names_map: HashMap<String, Vec<Rename>> = HashMap::new();

    for rename in files.drain(..) {
        match base_names_map.get_mut(&rename.new_stem) {
            None => {
                base_names_map.insert(rename.new_stem.clone(), vec![rename]);
            }
            Some(existing) => {
                existing.push(rename);
            }
        }
    }
    let mut base_names_list : Vec<(String, Vec<Rename>)>= base_names_map.drain().collect();
    base_names_list.sort_by(|x, y| x.0.cmp(&y.0));

    let mut non_clashes = Vec::with_capacity(files.len());
    let mut clashes = Vec::with_capacity(files.len());
    for (_, mut renames) in base_names_list.drain(..) {
        if renames.len() == 1 {
            non_clashes.append(&mut renames);
        }
        else {
            clashes.append(&mut renames);
        }
    }
    (non_clashes, clashes)
}
