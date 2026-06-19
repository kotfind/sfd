use time::UtcDateTime;
use time::format_description::well_known::Rfc3339 as Iso8601;

use crate::{
    context::DbContext,
    error::DbError,
    models::{embedding::Embedding, source_items::SourceItems},
};

/// Inserts a source.
pub async fn insert_source(
    db: DbContext,
    source_items: SourceItems,
    embeddings: &[Embedding],
    index_date: UtcDateTime,
) -> Result<(), DbError> {
    let path = source_items.src.path().to_str().expect("non-UTF-8 path");
    let index_date = index_date.format(&Iso8601).expect("infallible");

    let mut tx = db.pool().begin().await?;

    sqlx::query(
        "
        DELETE
        FROM source
        WHERE path = ?
        ",
    )
    .bind(path)
    .execute(&mut *tx)
    .await?;

    let result = sqlx::query(
        "
        INSERT
        INTO source (path, index_date)
        VALUES (?, ?)
        ",
    )
    .bind(path)
    .bind(&index_date)
    .execute(&mut *tx)
    .await?;

    let source_id = result.last_insert_rowid();

    for (item, embedding) in source_items.items.iter().zip(embeddings) {
        let vec_result = sqlx::query(
            "
            INSERT
            INTO vec (value)
            VALUES (?)
            ",
        )
        .bind(embedding.as_blob())
        .execute(&mut *tx)
        .await?;

        sqlx::query(
            "
            INSERT
            INTO item (source_id, item_offset, item_line_num, item_col_num, comment_content, comment_vec_id)
            VALUES (?, ?, ?, ?, ?, ?)
            ",
        )
        .bind(source_id)
        .bind(item.loc.offset as i64)
        .bind(item.loc.line_num as i64)
        .bind(item.loc.col_num as i64)
        .bind(item.comment.content())
        .bind(vec_result.last_insert_rowid())
        .execute(&mut *tx)
        .await?;
    }

    tx.commit().await?;

    Ok(())
}
