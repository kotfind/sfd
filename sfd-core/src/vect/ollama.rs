use reqwest::Client;
use serde::{Deserialize, Serialize};
use url::Url;

use crate::{config::spec::Config, models::embedding::Embedding};

use super::error::Error;

#[derive(Serialize)]
struct EmbeddingRequest<'a> {
    model: &'a str,
    prompt: &'a str,
}

#[derive(Deserialize)]
struct EmbeddingResponse {
    embedding: Vec<f32>,
}

/// Calls Ollama embedding model.
///
/// Use on already-prepared text (see [super::prepare::prepare]).
pub async fn embed_prepared(
    prompt: &str,
    config: &Config,
    client: &Client,
) -> Result<Embedding, Error> {
    let url = Url::parse(&config.vect.ollama.url)
        .expect("invalid ollama base url")
        .join("api/embeddings")
        .expect("invalid ollama embeddings endpoint");
    let resp = client
        .post(url)
        .json(&EmbeddingRequest {
            model: &config.vect.ollama.model,
            prompt,
        })
        .send()
        .await?;
    let data: EmbeddingResponse = resp.error_for_status()?.json().await?;
    Ok(Embedding::new(data.embedding))
}
