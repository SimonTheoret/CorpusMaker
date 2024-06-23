use crate::example::ToExample;

struct MonoDocument<T: ToExample> {
    examples: Vec<T>,
}

struct MultiDocuments {
    examples: Vec<Box<dyn ToExample>>,
}

struct DocumentBuilder<P: AsRef<std::path::Path>> {
    split: Option<&'static str>,
    directory: P,
    dir_walker: walkdir::IntoIter,
}

impl DocumentBuilder<P> {
    pub fn new(directory: ) -> Self {
        
    }
}
