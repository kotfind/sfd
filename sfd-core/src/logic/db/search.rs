use std::path::PathBuf;

use crate::{
    context::DbContext,
    error::DbError,
    models::{embedding::Embedding, location::Location, source::Source},
};

/// A search result.
#[derive(Debug, Clone)]
pub struct SearchResult {
    pub loc: Location,

    pub text: String,

    /// Semantic similarity to the query.
    ///
    /// Is in `[0; 1]` range.
    pub sim: f64,
}

/// Searches for the `limit` closest items to `embedding`.
pub async fn search(
    db: DbContext,
    embedding: &Embedding,
    limit: u32,
) -> Result<Vec<SearchResult>, DbError> {
    // TODO: rewrite with FromRow
    let rows: Vec<(i64, i64, String, String, f64)> =
        sqlx::query_as::<_, (i64, i64, String, String, f64)>(
            "
        SELECT item.item_line_num, item.item_col_num, item.comment_content, source.path, distance as dist
        FROM vec
        LEFT JOIN item ON item.comment_vec_id = vec.rowid
        LEFT JOIN source ON source.id = item.source_id
        WHERE vec.value MATCH ? AND k = ?
        -- automatically ORDERed BY dist
        ",
        )
        .bind(embedding.as_blob())
        .bind(limit as i64)
        .fetch_all(db.pool())
        .await?;

    Ok(rows
        .into_iter()
        .map(|(line_num, col_num, text, path, distance)| {
            let src = Source::new(PathBuf::from(path));
            SearchResult {
                loc: Location::new(src, 0, line_num as usize, col_num as usize),
                text,
                sim: 1.0 - distance / 2.0,
            }
        })
        .collect())
}
