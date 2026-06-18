use crate::{context::VectContext, error::VectError};

use super::{
    ping::ping,
    pull::{has_model, pull_model},
};

/// Ensures Ollama is reachable and the model is pulled.
pub async fn ensure_ready(ctx: VectContext) -> Result<(), VectError> {
    ping(ctx.clone()).await?;
    if !has_model(ctx.clone()).await? {
        pull_model(ctx).await?;
    }

    Ok(())
}
