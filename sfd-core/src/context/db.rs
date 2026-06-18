use std::{path::Path, str::FromStr};

use sqlx::sqlite::{SqliteConnectOptions, SqlitePool};

use crate::{config::Config, error::DbError, logic::db};

/// Database context.
#[derive(Debug, Clone)]
pub struct DbContext {
    pool: SqlitePool,
    _vec: db::VecExtLoadedProof,
}

impl DbContext {
    /// Creates a new db context, connecting to and initializing the db.
    pub async fn new(config: &Config) -> Result<Self, DbError> {
        let vec = db::load();

        let db_path = config
            .root()
            .join("sfd.db")
            .to_str()
            .expect("invalid db path")
            .to_owned();
        let is_new = !Path::new(&db_path).exists();

        let options = SqliteConnectOptions::from_str(&db_path)?.create_if_missing(true);
        let pool = SqlitePool::connect_with(options).await?;

        if is_new {
            db::init(&pool).await?;
        }

        Ok(Self { pool, _vec: vec })
    }

    pub fn pool(&self) -> &SqlitePool {
        &self.pool
    }
}
