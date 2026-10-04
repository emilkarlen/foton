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

// Copy of code in std::path (with modifications)
pub fn split_shortest_ext(file_name: &OsStr) -> (&OsStr, Option<&OsStr>)
{
    if file_name.as_encoded_bytes() == b".." {
        return (file_name, None);
    }

    // The unsafety here stems from converting between &OsStr and &[u8]
    // and back. This is safe to do because (1) we only look at ASCII
    // contents of the encoding and (2) new &OsStr values are produced
    // only from ASCII-bounded slices of existing &OsStr values.
    let mut iter = file_name.as_encoded_bytes().rsplitn(2, |b| *b == b'.');
    let after_match = iter.next();
    let before_match = iter.next();
    match before_match {
        None => (file_name, None),
        Some(before) => {
            if before == b"" {
                (file_name, None)
            }
            else {
                unsafe {
                    (OsStr::from_encoded_bytes_unchecked(before),
                     after_match.map(|s| OsStr::from_encoded_bytes_unchecked(s)))
                }
            }
        },
    }
}

// Copy of code in std::path
pub fn split_longest_ext(file: &OsStr) -> (&OsStr, Option<&OsStr>) {
    let slice = file.as_encoded_bytes();
    if slice == b".." {
        return (file, None);
    }

    // The unsafety here stems from converting between &OsStr and &[u8]
    // and back. This is safe to do because (1) we only look at ASCII
    // contents of the encoding and (2) new &OsStr values are produced
    // only from ASCII-bounded slices of existing &OsStr values.
    let i = match slice[1..].iter().position(|b| *b == b'.') {
        Some(i) => i + 1,
        None => return (file, None),
    };
    let before = &slice[..i];
    let after = &slice[i + 1..];
    unsafe {
        (
            OsStr::from_encoded_bytes_unchecked(before),
            Some(OsStr::from_encoded_bytes_unchecked(after)),
        )
    }
}

#[cfg(test)]
mod test
{
    use std::ffi::{OsStr, OsString};
    use crate::common::path::split_shortest_ext;

    #[test]
    fn test_split_shortest_ext()
    {
        fn str_to_os(a: &str, b: Option<&str>) -> (OsString, Option<OsString>) {
            (OsString::from(a), b.map(OsString::from))
        }
        fn os_to_os(x: (&OsStr, Option<&OsStr>)) -> (OsString, Option<OsString>) {
            (OsString::from(x.0), x.1.map(OsString::from))
        }

        fn run(name: &str) -> (OsString, Option<OsString>) {
            os_to_os(split_shortest_ext(OsString::from(name).as_os_str()))
        }
        assert_eq!(str_to_os("", None), run(""));
        assert_eq!(str_to_os(".", None), run("."));
        assert_eq!(str_to_os("..", None), run(".."));
        assert_eq!(str_to_os("non-hidden-no-ext", None), run("non-hidden-no-ext"));
        assert_eq!(str_to_os("non-hidden-1-ext", Some("e1")), run("non-hidden-1-ext.e1"));
        assert_eq!(str_to_os("non-hidden-2-ext.e1", Some("e2")), run("non-hidden-2-ext.e1.e2"));
        assert_eq!(str_to_os(".hidden-no-ext", None), run(".hidden-no-ext"));
        assert_eq!(str_to_os(".hidden-1-ext", Some("e1")), run(".hidden-1-ext.e1"));
        assert_eq!(str_to_os(".hidden-2-ext.e1", Some("e2")), run(".hidden-2-ext.e1.e2"));
    }
}
