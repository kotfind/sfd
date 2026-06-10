use crate::{db, extract, scan, vect};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("database error: {0}")]
    Db(#[from] db::error::Error),
    #[error("scan error: {0}")]
    Scan(#[from] scan::error::Error),
    #[error("extract error: {0}")]
    Extract(#[from] extract::error::Error),
    #[error("vect error: {0}")]
    Vect(#[from] vect::error::Error),
    #[error("http error: {0}")]
    Http(#[from] reqwest::Error),
}
