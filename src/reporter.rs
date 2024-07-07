use crate::err::*;
use crate::err_fmt;
use std::io;
use std::io::prelude::*;
use std::path::PathBuf;

pub struct Reporter
{
    rename: Box<dyn Write>,
    skip: Box<dyn Write>,
    io_error: Box<dyn Write>,
}


impl Reporter
{
    pub fn new() -> Self
    {
        Reporter
        {
            rename: Box::new(io::stdout()),
            skip: Box::new(io::stderr()),
            io_error: Box::new(io::stderr()),
        }
    }

    pub fn report_src_str_err(&mut self, file_name: &String, cause: &PathStrError)
    {
        let explanation = err_fmt::src_str_err(cause);
        self.report_skip(file_name, &explanation);
    }

    pub fn report_fs_error(&mut self, src: &String, dst: &PathBuf, cause: &PathFsError)
    {
        let explanation = err_fmt::fs_err(cause, dst);
        self.report_skip(src, &explanation);
    }

    pub fn report_rename(&mut self, src: &String, dst: &PathBuf)
    {
        let msg = format!("{} -> {}\n", src, dst.to_string_lossy());
        Reporter::write_to_stream(&mut self.rename,  msg)
    }

    pub fn report_stdin_read_error(&mut self, error: &io::Error)
    {
        let msg = format!("{}\n", error.to_string());
        Reporter::write_to_stream(&mut self.io_error,  msg)
    }

    fn report_skip(&mut self, file_name: &String, cause: &str)
    {
        let msg = err_fmt::format_skip(file_name, cause);
        Reporter::write_to_stream(&mut self.skip,  msg)
    }

    fn write_to_stream(stream: &mut Box<dyn Write>, msg: String)
    {
        match stream.write(msg.into_bytes().as_slice()) {
            Ok(_) => (),
            Err(io_error) => eprintln!("IO error while writing: {}", io_error),
        }
    }
}
