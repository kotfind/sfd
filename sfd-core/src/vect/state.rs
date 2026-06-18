use std::sync::Arc;

use reqwest::Client;
use url::Url;

use crate::config::spec::Config;

use super::error::Error;

/// Vectorization state.
#[derive(Debug, Clone)]
pub struct State {
    inner: Arc<StateInner>,
}

#[derive(Debug)]
struct StateInner {
    url: Url,
    model: String,
    client: Client,
    max_len: usize,
}

impl State {
    pub fn new(config: &Config) -> Result<Self, Error> {
        let url = Url::parse(&config.vect.ollama.url)?;
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs_f64(
                config.vect.ollama.timeout,
            ))
            .build()?;
        Ok(Self {
            inner: Arc::new(StateInner {
                url,
                model: config.vect.ollama.model.clone(),
                client,
                max_len: config.vect.max_len,
            }),
        })
    }

    pub(crate) fn url(&self) -> &Url {
        &self.inner.url
    }

    pub(crate) fn model(&self) -> &str {
        &self.inner.model
    }

    pub(crate) fn client(&self) -> &Client {
        &self.inner.client
    }

    pub(crate) fn max_len(&self) -> usize {
        self.inner.max_len
    }
}
