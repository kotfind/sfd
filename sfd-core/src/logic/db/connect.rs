use std::{path::Path, str::FromStr};

use sqlx::sqlite::{SqliteConnectOptions, SqlitePool};

use crate::{config::Config, context::DbContext, error::DbError};

use super::{
    init::{SCHEMA_HASH, init},
    load_extension::load,
};

/// Connects to db.
pub async fn connect(config: &Config) -> Result<DbContext, DbError> {
    let vec = load();

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
        init(&pool).await?;
    } else {
        let stored =
            sqlx::query_as::<_, (String,)>("SELECT value FROM setting WHERE key = 'schema_hash'")
                .fetch_one(&pool)
                .await?
                .0;

        if stored != *SCHEMA_HASH {
            return Err(DbError::SchemaMismatch);
        }
    }

    Ok(DbContext::new(pool, vec))
}
