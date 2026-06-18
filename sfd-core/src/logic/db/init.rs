use sha2::{Digest, Sha256};
use sqlx::sqlite::SqlitePool;

use crate::error::DbError;

const SCHEMA: &str = include_str!("schema.sql");

/// Inits db schema.
pub async fn init(pool: &SqlitePool) -> Result<(), DbError> {
    sqlx::query(SCHEMA).execute(pool).await?;

    let hash = hex::encode(Sha256::digest(SCHEMA));
    sqlx::query("INSERT INTO setting (key, value) VALUES ('schema_hash', ?)")
        .bind(&hash)
        .execute(pool)
        .await?;

    Ok(())
}
