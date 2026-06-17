/// Vectorization error.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("ollama request failed: {0}")]
    Http(#[from] reqwest::Error),
}
