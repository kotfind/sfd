use serde::{Deserialize, Serialize};

use crate::{context::VectContext, error::VectError, models::embedding::Embedding};

use super::prepare;

#[derive(Serialize)]
struct EmbedRequest<'a> {
    model: &'a str,
    input: &'a str,
}

#[derive(Deserialize)]
struct EmbedResponse {
    embeddings: Vec<Vec<f32>>,
}

/// Calls Ollama embedding model.
///
/// Use on already-prepared text (see [super::prepare::prepare]).
pub async fn embed_prepared(text: &str, ctx: VectContext) -> Result<Embedding, VectError> {
    let resp = ctx
        .client()
        .post(super::ping::route(&ctx, "api/embed"))
        .json(&EmbedRequest {
            model: ctx.model(),
            input: text,
        })
        .send()
        .await?;
    let data: EmbedResponse = resp.error_for_status()?.json().await?;
    Ok(Embedding::new(
        data.embeddings
            .into_iter()
            .next()
            .expect("expected exactly one embedding"),
    ))
}

/// Prepares text and gets its [Embedding].
pub async fn embed(text: &str, ctx: VectContext) -> Result<Embedding, VectError> {
    let text = prepare::prepare(text, ctx.clone());
    embed_prepared(&text, ctx).await
}
