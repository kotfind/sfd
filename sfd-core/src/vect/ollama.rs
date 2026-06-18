use serde::{Deserialize, Serialize};

use crate::models::embedding::Embedding;
use crate::vect::state::State;

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

fn route(state: &State, path: &str) -> url::Url {
    state.url().join(path).expect("the route is incorrect")
}

/// Ping Ollama API.
pub async fn ping(state: State) -> Result<(), Error> {
    state.client().get(state.url().clone()).send().await?;
    Ok(())
}

/// Is ollama model pulled?
pub async fn has_model(state: State) -> Result<bool, Error> {
    let tags: TagsResponse = state
        .client()
        .get(route(&state, "api/tags"))
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;
    Ok(tags.models.iter().any(|m| m.name == state.model()))
}

/// Pulls the model.
pub async fn pull_model(state: State) -> Result<(), Error> {
    state
        .client()
        .post(route(&state, "api/pull"))
        .json(&PullRequest {
            model: state.model(),
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
pub async fn embed_prepared(text: &str, state: State) -> Result<Embedding, Error> {
    let resp = state
        .client()
        .post(route(&state, "api/embed"))
        .json(&EmbedRequest {
            model: state.model(),
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
pub async fn embed(text: &str, state: State) -> Result<Embedding, Error> {
    let text = super::prepare::prepare(text, state.clone());
    embed_prepared(&text, state).await
}
