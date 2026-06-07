use crate::extract::{comment::Comment, ident::Ident, span::Span};

#[derive(Debug, Clone)]
pub struct Item {
    pub comment: Comment,

    pub ident: Ident,

    pub span: Span,
}

impl Item {
    pub fn new(comment: Comment, ident: Ident, span: Span) -> Self {
        Self {
            comment,
            ident,
            span,
        }
    }
}
