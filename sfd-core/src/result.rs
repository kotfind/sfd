use std::{collections::HashMap, path::PathBuf};

use time::UtcDateTime;

use crate::{
    error::FileExtractError,
    models::{location::Location, source::Source, source_items::SourceItems},
};

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

/// Result of an indexing run.
#[derive(Debug)]
pub struct IndexResult {
    pub date: UtcDateTime,

    pub sources: HashMap<Source, SourceItems>,

    pub errors: HashMap<Source, FileExtractError>,

    pub skipped: HashMap<PathBuf, SkipReason>,
}

/// Why a path was skipped during indexing.
#[derive(Debug)]
pub enum SkipReason {
    Ignored,
    Pattern,
    NoLang,
}
