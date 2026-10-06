use std::collections::HashMap;
use std::ffi::OsString;
use crate::common::read_files::files_builder::{FilesBuilder, FilesBuilderFactory};
use crate::common::read_files::types::StemAndExt;

pub type ExtToCount = HashMap<OsString, usize>;

pub fn new_factory() -> Box<dyn FilesBuilderFactory<ExtToCount>>
{
    Box::new(ExtsCountsFactory)
}

struct ExtsCounts
{
    counts: ExtToCount,
}

impl FilesBuilder<ExtToCount> for ExtsCounts
{
    fn add(&mut self, file: StemAndExt)
    {
        self.counts.entry(OsString::from(file.ext)).and_modify(|n| *n += 1).or_insert(1);
    }

    fn build(self: Box<Self>) -> ExtToCount
    {
        self.counts
    }
}

struct ExtsCountsFactory;

impl FilesBuilderFactory<ExtToCount> for ExtsCountsFactory
{
    fn new(&self) -> Box<dyn FilesBuilder<ExtToCount>>
    {
        Box::new(ExtsCounts{counts: HashMap::new()})
    }
}
