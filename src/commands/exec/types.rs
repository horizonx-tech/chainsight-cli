#[derive(Clone)]
pub enum ComponentsToInitialize {
    Single(String),
    Multiple(Vec<String>),
}
