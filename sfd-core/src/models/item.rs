use crate::models::{comment::Comment, ident::Ident, span::Span};

/// [Ident] with a [Comment] and a [Span].
#[derive(Debug, Clone)]
pub struct Item {
    pub comment: Comment,

    pub ident: Ident,

    /// A location of the `ident`.
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
