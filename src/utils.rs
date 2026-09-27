use std::ffi::OsStr;

pub fn from_os_str(s: &OsStr) -> String
{
    s.to_string_lossy().into_owned()
}

pub struct FixedWidthFormatter {
    pub width: usize,
}

impl FixedWidthFormatter {
    pub fn new_for_num(largest_num: usize) -> FixedWidthFormatter
    {
        FixedWidthFormatter {
            width: ((largest_num + 1) as f64).log10().ceil() as usize,
        }
    }
    pub fn new(width: usize) -> FixedWidthFormatter
    {
        FixedWidthFormatter {
            width
        }
    }

    pub fn num_0(& self, n: usize) -> String
    {
        let ret_val = format!("{:0width$}", n, width=self.width);
        ret_val
    }
    pub fn num_spc(& self, n: usize) -> String
    {
        let ret_val = format!("{:width$}", n, width=self.width);
        ret_val
    }
    pub fn str(& self, s: &str) -> String
    {
        let ret_val = format!("{:width$}", s, width=self.width);
        ret_val
    }
}
