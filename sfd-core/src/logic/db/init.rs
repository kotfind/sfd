use sha2::{Digest, Sha256};
use sqlx::sqlite::SqlitePool;
use std::sync::LazyLock;

use crate::error::DbError;

const SCHEMA: &str = include_str!("schema.sql");

/// Expected schema hash.
pub(crate) static SCHEMA_HASH: LazyLock<String> =
    LazyLock::new(|| hex::encode(Sha256::digest(SCHEMA)));

/// Inits db schema.
pub async fn init(pool: &SqlitePool, vec_size: usize) -> Result<(), DbError> {
    let mut tx = pool.begin().await?;

    let schema = prepare_schema(vec_size);
    sqlx::query(&schema).execute(&mut *tx).await?;

    sqlx::query("INSERT INTO setting (key, value) VALUES ('schema_hash', ?)")
        .bind(&*SCHEMA_HASH)
        .execute(&mut *tx)
        .await?;

    tx.commit().await?;

    Ok(())
}

/// Prepares the schema by replacing placeholders.
fn prepare_schema(vec_size: usize) -> String {
    SCHEMA.replace("__VEC_SIZE__", &vec_size.to_string())
}
