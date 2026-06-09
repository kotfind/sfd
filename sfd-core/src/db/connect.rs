use std::path::Path;

use sqlx::sqlite::SqlitePool;

use crate::config::spec::Config;

use super::{error::Error, init};

pub async fn connect(config: &Config) -> Result<SqlitePool, Error> {
    let db_path = config
        .root()
        .join("sfd.db")
        .to_str()
        .expect("invalid db path")
        .to_owned();
    let is_new = !Path::new(&db_path).exists();

    let pool = SqlitePool::connect(&db_path).await?;

    if is_new {
        init::init(&pool).await?;
    }

    Ok(pool)
}
