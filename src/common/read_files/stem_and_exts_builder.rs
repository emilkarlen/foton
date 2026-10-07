use super::files_builder::{FilesBuilder, FilesBuilderFactory};
use super::types::{Extension, FileNameStem, StemAndExt};
use std::collections::HashMap;

pub type StemToExtsMap = HashMap<FileNameStem, Vec<Extension>>;

pub fn stem_to_exts_map_builder() -> StemToExtsBuilderFactory
{
    StemToExtsBuilderFactory
}

struct StemToExtsBuilder
{
    value: StemToExtsMap,
}

impl StemToExtsBuilder
{
    pub fn new() -> StemToExtsBuilder
    {
        StemToExtsBuilder {value: HashMap::new()}
    }
}

impl FilesBuilder<StemToExtsMap> for StemToExtsBuilder
{
    fn add(&mut self, se: StemAndExt)
    {
        match self.value.get_mut(&se.stem) {
            None => {
                self.value.insert(se.stem, vec![se.ext]);
            }
            Some(exts) => {
                exts.push(se.ext);
            }
        }
    }
    fn build(self: Box<Self>) -> StemToExtsMap
    {
        let mut files = self.value;
        for exts in files.values_mut() {
            exts.sort();
        }
        files
    }
}

pub struct StemToExtsBuilderFactory;

impl FilesBuilderFactory<StemToExtsMap> for StemToExtsBuilderFactory
{
    fn new<'a>(&'a mut self) -> Box<dyn FilesBuilder<StemToExtsMap>>
    {
        Box::new(StemToExtsBuilder::new())
    }
}