use std::{path::Path, str::FromStr, time::Duration};

use sqlx::sqlite::{SqliteConnectOptions, SqlitePool};

use crate::{config::Config, context::DbContext, error::DbError};

use super::{
    init::{SCHEMA_HASH, init},
    load_extension::load,
};

/// Connects to db.
pub async fn connect(config: &Config, allow_create: bool) -> Result<DbContext, DbError> {
    let vec = load();

    let db_path = config
        .root()
        .join("sfd.db")
        .to_str()
        .expect("invalid db path")
        .to_owned();
    let is_new = !Path::new(&db_path).exists();

    if !allow_create && is_new {
        return Err(DbError::NotFound);
    }

    let options = SqliteConnectOptions::from_str(&db_path)?
        .create_if_missing(allow_create)
        .busy_timeout(Duration::from_secs(config.db.busy_timeout));
    let pool = SqlitePool::connect_with(options).await?;

    if is_new {
        init(&pool, config.vect.vec_size).await?;
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
