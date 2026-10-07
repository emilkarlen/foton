use super::types::ExtToCount;
use crate::common::read_files::files_builder::{FilesBuilder, FilesBuilderFactory};
use crate::common::read_files::types::StemAndExt;
use std::cell::{RefCell, RefMut};


pub struct ExtsCountsFactory
{
    pub extensions: RefCell<ExtToCount>,
}

impl ExtsCountsFactory
{
    pub fn new() -> Self
    {
        ExtsCountsFactory { extensions: RefCell::new(ExtToCount::new()) }
    }
}
impl FilesBuilderFactory<()> for ExtsCountsFactory
{
    fn new<'a>(&'a mut self) -> Box<dyn FilesBuilder<()> +'a>
    {
        Box::new(Builder { extensions: self.extensions.borrow_mut() })
    }
}

struct Builder<'a>
{
    extensions: RefMut<'a, ExtToCount>,
}

impl<'a> FilesBuilder<()> for Builder<'a>
{
    fn add(&mut self, file: StemAndExt)
    {
        self.extensions
            .entry(file.ext)
            .and_modify(|n| *n += 1)
            .or_insert(1);
    }

    fn build(self: Box<Self>) -> ()
    {
        ()
    }
}
