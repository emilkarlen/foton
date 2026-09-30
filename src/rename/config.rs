use crate::common::ext_filter::ExtensionsFilter;
use crate::rename::custom_format::FormatPart;

pub struct ReadConfig
{
    pub recursive: bool,
    pub extensions_filter: Box<dyn ExtensionsFilter>,
}

pub struct NamingConfigCli
{
    pub start_num: usize,
    pub min_width: usize,
    pub format: Option<String>,
}

pub struct NamingConfig
{
    pub start_num: usize,
    pub min_width: usize,
    pub format: Option<Vec<FormatPart>>,
}
