use thiserror;
use tree_sitter::{LanguageError, QueryError, WasmError};

use crate::{
    error::FileExtractError,
    logic::extract::{COMMENT_CAPTURE, ITEM_CAPTURE},
};

/// Extraction error.
#[derive(Debug, thiserror::Error)]
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

    #[error("per-file extraction error: {0}")]
    File(#[from] FileExtractError),
}
