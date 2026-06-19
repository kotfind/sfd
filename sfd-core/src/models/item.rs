use crate::models::{comment::Comment, ident::Ident, location::Location};

/// [Ident] with a [Comment] and a [Location].
#[derive(Debug, Clone)]
pub struct Item {
    pub comment: Comment,

    pub ident: Ident,

    /// A location of the `ident`.
    pub loc: Location,
}

impl Item {
    pub fn new(comment: Comment, ident: Ident, loc: Location) -> Self {
        Self {
            comment,
            ident,
            loc,
        }
    }
}
