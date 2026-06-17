use std::{ops::RangeInclusive, path::PathBuf};

use thiserror::Error;
use tree_sitter::{LanguageError, QueryError, WasmError};

use crate::extract::extract_items::{COMMENT_CAPTURE, ITEM_CAPTURE};

/// Extraction error.
#[derive(Debug, Error)]
pub enum Error {
    #[error("failed to perform io operation: {0}")]
    Io(#[from] std::io::Error),

    #[error("failed to create wasm store: {0}")]
    Wasm(#[from] WasmError),

    #[error("failed to load language: {0}")]
    Language(#[from] LanguageError),

    #[error("failed to compile query: {0}")]
    Query(#[from] QueryError),

    #[error(
        "query must have exactly one @{COMMENT_CAPTURE} and exactly one @{ITEM_CAPTURE} capture"
    )]
    InvalidQuery,

    #[error("unexpected number of @{name} captures (expected {expected:?}, got {actual})")]
    UnexpectedCaptureCount {
        name: String,
        expected: RangeInclusive<usize>,
        actual: usize,
    },

    #[error("no language detected for `{path}`")]
    NoLang { path: PathBuf },

    #[error("source file contains syntax errors")]
    SyntaxError,

    #[error("tree-sitter returned a non-utf8 slice")]
    NonUtf8,
}

impl Error {
    /// Was this error caused by a single file's contents?
    pub fn is_file_local(&self) -> bool {
        matches!(
            self,
            Error::NoLang { .. } | Error::SyntaxError | Error::NonUtf8
        )
    }
}
