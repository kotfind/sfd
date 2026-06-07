use thiserror::Error;
use tree_sitter::{LanguageError, QueryError, WasmError};

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
}
