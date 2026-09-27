use std::ffi::{OsStr, OsString};
use std::ops::Deref;

pub trait ExtensionsFilter {
    fn accept(&self, extension: &str) -> bool;
    fn accepts_os(&self, extension: &OsStr) -> bool;
}

pub fn any() -> impl ExtensionsFilter
{
    AcceptAll
}

pub fn exclude(extensions: Vec<Box<String>>) -> impl ExtensionsFilter
{
    Neg{ filter: Box::new(include(extensions)) }
}

pub fn include(extensions: Vec<Box<String>>) -> impl ExtensionsFilter
{
    Include::from(extensions)
}

impl ExtensionsFilter for AcceptAll
{
    fn accept(&self, _extension: &str) -> bool { true }
    fn accepts_os(&self, _extension: &OsStr) -> bool { true }
}

impl ExtensionsFilter for Include
{
    fn accept(&self, extension: &str) -> bool
    {
        self.included.iter().find(|x| ***x == extension).is_some()
    }

    fn accepts_os(&self, extension: &OsStr) -> bool
    {
        self.included_os.iter().find(|x| **x == extension).is_some()
    }
}

impl ExtensionsFilter for Neg
{
    fn accept(&self, extension: &str) -> bool { !self.filter.accept(extension) }
    fn accepts_os(&self, extension: &OsStr) -> bool {
        !self.filter.accepts_os(extension)
    }
}

struct AcceptAll;

struct Neg
{
    filter: Box<dyn ExtensionsFilter>
}


struct Include
{
    included: Vec<Box<String>>,
    included_os: Vec<OsString>,
}

impl Include
{
    fn from(x: Vec<Box<String>>) -> Self
    {
        Include {
            included_os: x.iter().map(|s| OsString::from(s.deref())).collect(),
            included: x,
        }
    }
}
