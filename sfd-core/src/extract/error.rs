use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("failed to read {path}: {source}")]
    Io {
        path: String,

        #[source]
        source: std::io::Error,
    },
}
