pub trait ExtensionsFilter {
    fn accept(&self, extension: &str) -> bool;
}

pub fn any() -> impl ExtensionsFilter
{
    AcceptAll
}

pub fn exclude(extensions: Vec<Box<String>>) -> impl ExtensionsFilter
{
    Exclude{ excluded: extensions }
}

pub fn include(extensions: Vec<Box<String>>) -> impl ExtensionsFilter
{
    Include{ included: extensions }
}

impl ExtensionsFilter for AcceptAll
{
    fn accept(&self, _extension: &str) -> bool { true }
}

impl ExtensionsFilter for Exclude
{
    fn accept(&self, extension: &str) -> bool { !self.excluded.iter().find(|x| ***x == extension).is_some() }
}

impl ExtensionsFilter for Include
{
    fn accept(&self, extension: &str) -> bool { self.included.iter().find(|x| ***x == extension).is_some() }
}

struct AcceptAll;

struct Exclude
{
    excluded: Vec<Box<String>>
}

struct Include
{
    included: Vec<Box<String>>
}
