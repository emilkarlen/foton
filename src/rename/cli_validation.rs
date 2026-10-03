use crate::common::arg_validation;
use crate::common::read_files::PathWithName;
use crate::rename::config::{NamingConfig, NamingConfigCli};
use crate::rename::custom_format::FormatPart;
use crate::rename::custom_format;
use std::path::PathBuf;

pub fn validate_args(directories: &Vec<PathBuf>, naming_config: &NamingConfigCli) -> Result<(Vec<PathWithName>, NamingConfig), String>
{
    let dirs = check_dirs(directories)?;
    let naming_config = resolve_and_check_naming_args(naming_config)?;
    Ok((dirs, naming_config))
}

fn check_dirs(directories: &Vec<PathBuf>) -> Result<Vec<PathWithName>, String>
{
    for dir in directories.iter() {
        arg_validation::is_existing_dir(dir.as_ref())?;
    }
    let mut dirs = Vec::with_capacity(directories.len());
    for dir in directories.iter() {
        let pwn = arg_validation::is_path_with_name(dir, "DIR")?;
        dirs.push(pwn);
    }
    Ok(dirs)
}

fn resolve_and_check_naming_args(config: &NamingConfigCli) -> Result<NamingConfig, String>
{
    match config.format.as_ref() {
        None => nc_of(config, None),
        Some(format_str) => {
            let format_parts = custom_format::parse(&format_str)?;
            check_format(&format_parts)?;
            nc_of(config, Some(format_parts))
        }
    }
}

fn check_format(format_parts: &Vec<FormatPart>) -> Result<(), String>
{
    fn check_part(part: &FormatPart) -> Result<(), String>
    {
        match part {
            FormatPart::Derived(_) => Ok(()),
            FormatPart::Const(str) => check_format_part_const(str),
        }
    }
    format_parts.iter().map(check_part).collect::<Result<(), String>>()?;
    Ok(())
}

fn check_format_part_const(part: &String) -> Result<(), String>
{
    if let Some(_) = part.find(std::path::MAIN_SEPARATOR) {
        Err(format!("Format must not contain directory separators: {}", part))
    }
    else {
        Ok(())
    }
}

fn nc_of(config: &NamingConfigCli, format: Option<Vec<FormatPart>>) -> Result<NamingConfig, String> {
    Ok(nc_of_plain(config, format))
}

fn nc_of_plain(config: &NamingConfigCli, format: Option<Vec<FormatPart>>) -> NamingConfig {
    NamingConfig {
        start_num: config.start_num,
        min_width: config.min_width,
        format: format,
    }    }
