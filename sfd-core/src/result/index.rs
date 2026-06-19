use std::{collections::HashMap, path::PathBuf};

use time::UtcDateTime;

use crate::{
    error::FileExtractError,
    models::{skip_reason::SkipReason, source::Source, source_items::SourceItems},
};

/// Result of an indexing run.
#[derive(Debug)]
pub struct IndexResult {
    pub date: UtcDateTime,

    pub sources: HashMap<Source, SourceItems>,

    pub errors: HashMap<Source, FileExtractError>,

    pub skipped: HashMap<PathBuf, SkipReason>,
}
