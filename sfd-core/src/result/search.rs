use crate::models::location::Location;

/// A search result.
#[derive(Debug, Clone)]
pub struct SearchResult {
    pub loc: Location,

    pub text: String,

    /// Semantic similarity to the query.
    ///
    /// Is in `[0; 1]` range.
    pub sim: f64,
}
