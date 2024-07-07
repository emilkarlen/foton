use std::io::Stdin;
use std::path::{Path, PathBuf};

use crate::err::*;
use crate::renamer::*;
use crate::reporter::*;


pub fn rename_files(file_names_reader: Stdin, renamer: Renamer,reporter: &mut Reporter) -> std::process::ExitCode
{
    let mut exit_code = std::process::ExitCode::SUCCESS;

    for input_line_r in file_names_reader.lines() {
        match input_line_r {
            Err(io_err) => {
                reporter.report_stdin_read_error(&io_err);
                exit_code = std::process::ExitCode::FAILURE;
                break;
            }
            Ok(input_line) => {
                process_file_name(&renamer, reporter, &input_line);
            }
        }
    }
    exit_code
}


fn process_file_name(renamer: &Renamer, reporter: &mut Reporter, src: &String)
{
    let src_p = PathBuf::from(src);
    match analyze_path_string(&src_p) {
        Err(err) => {
            reporter.report_src_str_err(src, &err);
        }
        Ok(dst_p) => {
            let rename = Rename { src: src_p, dst: dst_p };
            match renamer.apply(&rename) {
                Err(err) => {
                    reporter.report_fs_error(src, &rename.dst, &err);
                }
                Ok(_) => {
                    reporter.report_rename(src, &rename.dst);
                }
            }
        }
    }
}


fn analyze_path_string(path: &PathBuf) -> Result<PathBuf, PathStrError>
{
    let file_name = path.file_name()
        .ok_or_else(|| PathStrError::MissingFileName)
        .map(Path::new)?;

    let ext_os_str = file_name.extension()
        .ok_or_else(|| PathStrError::MissingExtension)?;

    let ext_str = ext_os_str.to_str()
        .ok_or_else(|| PathStrError::NonUnicodeExtension)?;

    let ext_str_lower = String::from(ext_str).to_lowercase();
    if ext_str_lower.eq(ext_str) {
        Err(PathStrError::LowercaseExtension)
    } else {
        let path_w_lowercase_ext = path.with_extension(ext_str_lower);
        Ok(path_w_lowercase_ext)
    }
}
