use std::ops::RangeInclusive;

use thiserror;

/// Per-file extraction error.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("unexpected number of @{name} captures (expected {expected:?}, got {actual})")]
    UnexpectedCaptureCount {
        name: String,
        expected: RangeInclusive<usize>,
        actual: usize,
    },

    #[error("source file contains syntax errors")]
    SyntaxError,

    #[error("tree-sitter returned a non-utf8 slice")]
    NonUtf8,

    #[error("no language detected")]
    NoLang,
}
