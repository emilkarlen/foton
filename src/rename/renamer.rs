use crate::rename::common::FnInfo;
use crate::rename::config::NamingConfig;
use crate::rename::custom_format::{FormatPart, Property};
use crate::rename::renamer::num_generator::SequentialNumGenerator;
use crate::common::read_files;

mod num_generator;

pub trait StemGenerator
{
    fn next(&mut self) -> String;

    fn may_produce_clashes(&self) -> bool;
}


pub fn resolve(dc: &read_files::DirContents<Vec<FnInfo>>, config: &NamingConfig) -> Box<dyn StemGenerator>
{
    let num_gen = num_generator_for(dc, config);
    if let Some(fps) = config.format.as_ref() {
        // TODO clone: should not need to clone here
        Box::new(FormatGenerator{ format: fps.clone(), num_generator: num_gen })
    } else {
        Box::new(NumOnlyGenerator{ generator: num_gen })
    }
}

struct NumOnlyGenerator
{
    generator: SequentialNumGenerator,
}

impl StemGenerator for NumOnlyGenerator
{
    fn next(&mut self) -> String
    {
        self.generator.format()
    }

    fn may_produce_clashes(&self) -> bool
    {
        false
    }
}

struct FormatGenerator
{
    // TODO clone: should not need to clone here: use ref - but requires lifetimes
    format: Vec<FormatPart>,
    num_generator: SequentialNumGenerator,
}

impl StemGenerator for FormatGenerator
{
    fn next(&mut self) -> String
    {
        let next_num =self.num_generator.format();
        let mut ret_val = String::new();
        for fp in self.format.iter() {
            match fp {
                FormatPart::Const(s) => ret_val.push_str(s.as_str()),
                FormatPart::Derived(prop) => {
                    match prop {
                        Property::Number => ret_val.push_str(&next_num)
                    }
                }
            }
        }
        ret_val
    }

    fn may_produce_clashes(&self) -> bool
    {
        fn guaranties_no_clashes(x: &&FormatPart) -> bool
        {
            match x {
                FormatPart::Derived(Property::Number) => true,
                _ => false
            }
        }
        !self.format.iter().find(guaranties_no_clashes).is_some()
    }
}
pub fn num_generator_for(dc: &read_files::DirContents<Vec<FnInfo>>, config: &NamingConfig) -> SequentialNumGenerator
{
    let num_stems = num_stems_in(dc);
    let max_stem_number = {
        let x = config.start_num + num_stems;
        if x == 0 { x } else { x-1 }
    };
    SequentialNumGenerator::new(config.start_num, max_stem_number, config.min_width)
}

fn num_stems_in(x: &read_files::DirContents<Vec<FnInfo>>) -> usize
{
    let mut ret_val = x.files.len();
    for sub_dir in x.sub_dirs.iter() {
        ret_val += num_stems_in(sub_dir);
    }
    ret_val
}