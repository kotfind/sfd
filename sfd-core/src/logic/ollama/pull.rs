use serde::Deserialize;

use crate::{context::VectContext, error::VectError};

#[derive(Deserialize)]
struct TagsResponse {
    models: Vec<ModelInfo>,
}

#[derive(Deserialize)]
struct ModelInfo {
    name: String,
}

#[derive(serde::Serialize)]
struct PullRequest<'a> {
    model: &'a str,
    stream: bool,
}

/// Is ollama model pulled?
pub async fn has_model(ctx: VectContext) -> Result<bool, VectError> {
    let tags: TagsResponse = ctx
        .client()
        .get(super::ping::route(&ctx, "api/tags"))
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;
    Ok(tags.models.iter().any(|m| m.name == ctx.model()))
}

/// Pulls the model.
pub async fn pull_model(ctx: VectContext) -> Result<(), VectError> {
    ctx.client()
        .post(super::ping::route(&ctx, "api/pull"))
        .json(&PullRequest {
            model: ctx.model(),
            stream: false,
        })
        .send()
        .await?
        .error_for_status()?;
    Ok(())
}
