/// Vectorization error.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("invalid ollama base url: {0}")]
    InvalidUrl(#[from] url::ParseError),

    #[error("ollama request failed: {0}")]
    Http(#[from] reqwest::Error),
}
