use std::sync::Arc;

use reqwest::Client;
use url::Url;

use crate::{config::Config, error::VectError, logic::ollama};

/// Vectorization context.
#[derive(Debug, Clone)]
pub struct VectContext {
    inner: Arc<VectContextInner>,
}

#[derive(Debug)]
struct VectContextInner {
    url: Url,
    model: String,
    client: Client,
    max_len: usize,
    vec_size: usize,
}

impl VectContext {
    pub async fn new(config: &Config) -> Result<Self, VectError> {
        let url = Url::parse(&config.vect.ollama.url)?;
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs_f64(
                config.vect.ollama.timeout,
            ))
            .build()?;
        let ctx = Self {
            inner: Arc::new(VectContextInner {
                url,
                model: config.vect.ollama.model.clone(),
                client,
                max_len: config.vect.max_len,
                vec_size: config.vect.vec_size,
            }),
        };

        ollama::ensure_ready(ctx.clone()).await?;

        Ok(ctx)
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

    pub(crate) fn vec_size(&self) -> usize {
        self.inner.vec_size
    }
}
