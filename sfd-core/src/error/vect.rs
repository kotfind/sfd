/// Vectorization error.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("invalid ollama base url: {0}")]
    InvalidUrl(#[from] url::ParseError),

    #[error("ollama request failed: {0}")]
    Http(#[from] reqwest::Error),

    #[error(
        "embedding vector size mismatch: config says {config_value}, model returned {model_value}"
    )]
    VectSize {
        config_value: usize,
        model_value: usize,
    },
}
