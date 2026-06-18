/// Scanning error.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("invalid glob pattern: {0}")]
    Glob(#[from] globset::Error),

    #[error("failed to walk directory: {0}")]
    Walk(#[from] ignore::Error),

    #[error("extraction failed: {0}")]
    Extract(#[from] super::ExtractError),
}
