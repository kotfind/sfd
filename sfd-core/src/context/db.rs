use derive_more::Debug;
use sqlx::sqlite::SqlitePool;

use crate::logic::db::VecExtLoadProof;

/// Database context.
#[derive(Debug, Clone)]
pub struct DbContext {
    pool: SqlitePool,

    #[debug(skip)]
    _vec: VecExtLoadProof,
}

impl DbContext {
    pub fn new(pool: SqlitePool, vec: VecExtLoadProof) -> Self {
        Self { pool, _vec: vec }
    }

    pub fn pool(&self) -> &SqlitePool {
        &self.pool
    }
}
