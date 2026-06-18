mod config;
mod db;
mod extract;
mod scan;
mod vect;

pub use config::Error as ConfigError;
pub use db::Error as DbError;
pub use extract::Error as ExtractError;
pub use scan::Error as ScanError;
pub use vect::Error as VectError;

/// Top-level error.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("database error: {0}")]
    Db(#[from] DbError),

    #[error("scan error: {0}")]
    Scan(#[from] ScanError),

    #[error("extract error: {0}")]
    Extract(#[from] ExtractError),

    #[error("vect error: {0}")]
    Vect(#[from] VectError),

    #[error("http error: {0}")]
    Http(#[from] reqwest::Error),
}
