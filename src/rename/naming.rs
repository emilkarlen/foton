use std::collections::HashMap;
use std::path::PathBuf;
use crate::command::{ExeError, UnableToExecuteError};
use crate::common::read_files::DirContents;
use crate::rename::common::{FnInfo, Rename};
use crate::rename::config::NamingConfig;
use crate::rename::renamer;
use crate::rename::renamer::StemGenerator;

const REASON: &str = "Name clashes";

pub fn resolve(dc: DirContents<Vec<FnInfo>>, config: &NamingConfig) -> Result<DirContents<Vec<Rename>>, ExeError>
{
    let mut name_generator = renamer::resolve(&dc, config);
    let renames = renames_of(dc, &mut name_generator);
    if name_generator.may_produce_clashes() {
        with_check_for_clashes(renames)
    }
    else {
            Ok(renames)
    }
}

fn with_check_for_clashes(renames: DirContents<Vec<Rename>>) -> Result<DirContents<Vec<Rename>>, ExeError>
{
    let mut errs = Vec::new();
    let renames = check_clashes(renames, &mut errs);
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
fn renames_of(dc: DirContents<Vec<FnInfo>>, stem_gen: &mut Box<dyn StemGenerator>) -> DirContents<Vec<Rename>>
{
    let DirContents { dir, mut sub_dirs, mut files} = dc;

    let mut files_2 = Vec::with_capacity(files.len());
    for fni in files.drain(..) {
        let ren = Rename {
            new_stem: stem_gen.next(),
            old_stem: fni.stem,
            extensions: fni.extensions,
        };
        files_2.push(ren);

    }
    let mut sub_dirs_2 = Vec::with_capacity(sub_dirs.len());
    for sub_dir in sub_dirs.drain(..) {
        sub_dirs_2.push(renames_of(sub_dir, stem_gen))
    }
    DirContents {
        dir: dir,
        sub_dirs: sub_dirs_2,
        files: files_2,
    }
}

fn check_clashes(dc: DirContents<Vec<Rename>>, errs: &mut Vec<(PathBuf, Vec<Rename>)>) -> DirContents<Vec<Rename>>
{
    fn process_sub_dirs(sub_dirs: Vec<DirContents<Vec<Rename>>>, errs: &mut Vec<(PathBuf, Vec<Rename>)>) -> Vec<DirContents<Vec<Rename>>>
    {
        sub_dirs.into_iter().map(|x| check_clashes(x, errs)).collect()
    }

    let DirContents { dir, sub_dirs, mut files } = dc;
    let (non_clashes, clashes) = check_files(&mut files);
    if !clashes.is_empty() {
        errs.push((dir.path.clone(), clashes));
    }
    DirContents {
        dir: dir,
        files: non_clashes,
        sub_dirs: process_sub_dirs(sub_dirs, errs),
    }
}

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
