use crate::file_exts::{read_files, reporting};
use std::io;

pub fn main(config: &crate::file_exts::CmdConfig,
) -> io::Result<()>
{
    if config.read_config.recursive && config.depth > 0  {
        rec_depth_non_0(config)
    }
    else {
        depth0(config)
    }
}

fn depth0(config: &crate::file_exts::CmdConfig) -> io::Result<()>
{
    let mut extensions = read_files::single_global_count(config.directories.clone(), &config.read_config)?;
    reporting::global(&config.report_config, &mut extensions);
    Ok(())
}

fn rec_depth_non_0(config: &crate::file_exts::CmdConfig) -> io::Result<()>
{
    let extensions_per_dir = read_files::counter_per_dir(config.directories.clone(), &config.read_config)?;
    reporting::per_node(config.depth, &config.report_config, extensions_per_dir);
    Ok(())
}
