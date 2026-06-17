use crate::models::{item::Item, source::Source};

/// A [`Source`] with [`Item`]s extracted from it.
#[derive(Debug, Clone)]
pub struct SourceItems {
    pub src: Source,

    pub items: Vec<Item>,
}

impl SourceItems {
    pub fn new(src: Source, items: Vec<Item>) -> Self {
        Self { src, items }
    }
}
