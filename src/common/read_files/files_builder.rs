use super::types::StemAndExt;

pub trait FilesBuilder<T>
{
    fn add(&mut self, file: StemAndExt);

    fn build(self: Box<Self>) -> T;
}

pub trait FilesBuilderFactory<T>
{
    fn new(&self) -> Box<dyn FilesBuilder<T>>;
}
