use clap::Parser;
use sfd::{Context, config::Config};

/// CLI arguments.
#[derive(Parser)]
struct Args {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _args = Args::parse();

    let config = Config::load()?;
    let ctx = Context::new(&config).await?;
    ctx.run().await?;

    Ok(())
}
