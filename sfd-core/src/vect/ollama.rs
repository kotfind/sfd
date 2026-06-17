use reqwest::Client;
use serde::{Deserialize, Serialize};
use url::Url;

use crate::{config::spec::Config, models::embedding::Embedding};

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

/// Calls Ollama embedding model.
///
/// Use on already-prepared text (see [super::prepare::prepare]).
pub async fn embed_prepared(
    text: &str,
    config: &Config,
    client: &Client,
) -> Result<Embedding, Error> {
    let url = Url::parse(&config.vect.ollama.url)
        .expect("invalid ollama base url")
        .join("api/embed")
        .expect("invalid ollama embed endpoint");
    let resp = client
        .post(url)
        .json(&EmbedRequest {
            model: &config.vect.ollama.model,
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
