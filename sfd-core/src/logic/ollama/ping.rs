use crate::{context::VectContext, error::VectError};

pub(crate) fn route(ctx: &VectContext, path: &str) -> url::Url {
    ctx.url().join(path).expect("the route is incorrect")
}

/// Ping Ollama API.
pub async fn ping(ctx: VectContext) -> Result<(), VectError> {
    ctx.client().get(ctx.url().clone()).send().await?;
    Ok(())
}
