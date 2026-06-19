use clap::Parser;
use schematic::schema::{SchemaGenerator, SchemaRenderer, TemplateOptions, YamlTemplateRenderer};
use sfd_core::config::Config;
use tokio::io::AsyncWriteExt;

use crate::error::Error;

/// Outputs a sample config.
#[derive(Parser)]
pub struct SampleConfigCmd;

pub async fn run(_cmd: SampleConfigCmd) -> Result<(), Error> {
    let mut generator = SchemaGenerator::default();
    generator.add::<Config>();
    let mut renderer = YamlTemplateRenderer::new(TemplateOptions::default());
    let yaml = renderer
        .render(generator.schemas)
        .map_err(Error::SchemaRender)?;
    tokio::io::stdout().write_all(yaml.as_bytes()).await?;

    Ok(())
}
