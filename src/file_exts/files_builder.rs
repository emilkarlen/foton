use crate::common::read_files::files_builder::{FilesBuilder, FilesBuilderFactory};
use crate::common::read_files::types::StemAndExt;
use crate::file_exts::types::ExtToCount;
use std::collections::HashMap;

pub struct ExtsCountsFactory;

pub fn new_factory() -> ExtsCountsFactory
{
    ExtsCountsFactory
}

struct ExtsCounts
{
    counts: ExtToCount,
}

impl FilesBuilder<ExtToCount> for ExtsCounts
{
    fn add(&mut self, file: StemAndExt)
    {
        self.counts
            .entry(file.ext)
            .and_modify(|n| *n += 1)
            .or_insert(1);
    }

    fn build(self: Box<Self>) -> ExtToCount
    {
        self.counts
    }
}

impl FilesBuilderFactory<ExtToCount> for ExtsCountsFactory
{
    fn new<'a>(&'a mut self) -> Box<dyn FilesBuilder<ExtToCount> +'a>
    {
        Box::new(ExtsCounts{counts: HashMap::new()})
    }
}
