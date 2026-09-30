use crate::rename::custom_format::FormatPart;

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
