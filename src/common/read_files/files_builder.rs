use super::types::StemAndExt;

pub trait FilesBuilder<T>
{
    fn add(&mut self, file: StemAndExt);

    fn build(self: Box<Self>) -> T;
}

pub trait FilesBuilderFactory<T>
{
    fn new<'a>(&'a mut self) -> Box<dyn FilesBuilder<T> + 'a>;
}
