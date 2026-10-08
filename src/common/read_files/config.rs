use crate::common::ext_filter::ExtensionsFilter;
use crate::common::read_files::types::StemExtSplitter;

pub struct ReadConfig
{
    pub recursive: bool,
    pub include_hidden_sub_dirs: bool,
    pub extensions_filter: Box<dyn ExtensionsFilter>,
    pub split_stem_and_ext: StemExtSplitter,
}
