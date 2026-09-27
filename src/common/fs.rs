use crate::utils;
use std::ffi::OsStr;

pub fn is_hidden(name: &OsStr) -> bool
{
    if let Some(ch) = utils::from_os_str(name).chars().next() {
        ch == '.'
    } else {
        // string is empty
        true
    }
}
