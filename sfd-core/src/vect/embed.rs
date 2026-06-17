use reqwest::Client;

use crate::{config::spec::Config, models::embedding::Embedding};

use super::{error::Error, ollama, prepare};

/// Turns a piece of text into an [Embedding].
pub async fn embed(text: &str, config: &Config, client: &Client) -> Result<Embedding, Error> {
    let text = prepare::prepare(text, config);
    ollama::embed_prepared(&text, config, client).await
}
