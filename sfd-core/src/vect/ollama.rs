use serde::{Deserialize, Serialize};

use crate::models::embedding::Embedding;
use crate::vect::state::VectContext;

use super::error::Error;

#[derive(Serialize)]
struct EmbedRequest<'a> {
    model: &'a str,
    input: &'a str,
}

#[derive(Deserialize)]
struct EmbedResponse {
    embeddings: Vec<Vec<f32>>,
}

#[derive(Deserialize)]
struct TagsResponse {
    models: Vec<ModelInfo>,
}

#[derive(Deserialize)]
struct ModelInfo {
    name: String,
}

#[derive(Serialize)]
struct PullRequest<'a> {
    model: &'a str,
    stream: bool,
}

fn route(ctx: &VectContext, path: &str) -> url::Url {
    ctx.url().join(path).expect("the route is incorrect")
}

/// Ping Ollama API.
pub async fn ping(ctx: VectContext) -> Result<(), Error> {
    ctx.client().get(ctx.url().clone()).send().await?;
    Ok(())
}

/// Is ollama model pulled?
pub async fn has_model(ctx: VectContext) -> Result<bool, Error> {
    let tags: TagsResponse = ctx
        .client()
        .get(route(&ctx, "api/tags"))
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;
    Ok(tags.models.iter().any(|m| m.name == ctx.model()))
}

/// Pulls the model.
pub async fn pull_model(ctx: VectContext) -> Result<(), Error> {
    ctx.client()
        .post(route(&ctx, "api/pull"))
        .json(&PullRequest {
            model: ctx.model(),
            stream: false,
        })
        .send()
        .await?
        .error_for_status()?;
    Ok(())
}

/// Calls Ollama embedding model.
///
/// Use on already-prepared text (see [super::prepare::prepare]).
pub async fn embed_prepared(text: &str, ctx: VectContext) -> Result<Embedding, Error> {
    let resp = ctx
        .client()
        .post(route(&ctx, "api/embed"))
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
pub async fn embed(text: &str, ctx: VectContext) -> Result<Embedding, Error> {
    let text = super::prepare::prepare(text, ctx.clone());
    embed_prepared(&text, ctx).await
}
