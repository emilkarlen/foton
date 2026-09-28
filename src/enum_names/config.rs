use crate::common::ext_filter::ExtensionsFilter;

pub struct ReadConfig
{
    pub recursive: bool,
    pub extensions_filter: Box<dyn ExtensionsFilter>,
}

pub struct NamingConfig
{
    pub start_num: usize,
    pub min_width: usize,
}
