use crate::extract::{comment::Comment, ident::Ident};

#[derive(Debug, Clone)]
pub struct Symbol {
    pub name: Ident,
    pub comment: Option<Comment>,
}

impl Symbol {
    pub fn new(name: Ident, comment: Option<Comment>) -> Self {
        Self { name, comment }
    }
}
