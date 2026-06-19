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
    let vec_size = ctx.vec_size();

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

    let embeddings = data
        .embeddings
        .into_iter()
        .map(|e| {
            if e.len() == vec_size {
                return Ok(Embedding::new(e));
            }
            Err(VectError::VectSize {
                config_value: vec_size,
                model_value: e.len(),
            })
        })
        .collect::<Result<Vec<_>, _>>()?;

    Ok(embeddings)
}
