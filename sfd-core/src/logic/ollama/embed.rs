use serde::{Deserialize, Serialize};

use crate::{context::VectContext, error::VectError, models::embedding::Embedding};

use super::prepare::prepare;

#[derive(Serialize)]
struct EmbedRequest<'a> {
    model: &'a str,
    input: Vec<String>,
}

#[derive(Deserialize)]
struct EmbedResponse {
    embeddings: Vec<Vec<f32>>,
}

/// Prepares texts and gets their [Embedding]s in a single request.
pub async fn embed(
    texts: impl IntoIterator<Item = impl AsRef<str>>,
    ctx: VectContext,
) -> Result<Vec<Embedding>, VectError> {
    let input: Vec<String> = texts
        .into_iter()
        .map(|t| prepare(t.as_ref(), ctx.clone()))
        .collect();

    let resp = ctx
        .client()
        .post(super::ping::route(&ctx, "api/embed"))
        .json(&EmbedRequest {
            model: ctx.model(),
            input,
        })
        .send()
        .await?;
    let data: EmbedResponse = resp.error_for_status()?.json().await?;

    Ok(data.embeddings.into_iter().map(Embedding::new).collect())
}
