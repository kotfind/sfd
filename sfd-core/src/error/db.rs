/// Db error.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("database error: {0}")]
    Sqlx(#[from] sqlx::Error),

    #[error("schema hash mismatch")]
    SchemaMismatch,

    #[error("database not found")]
    NotFound,
}
