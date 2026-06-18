use clap::Parser;
use sfd_core::{Client, config::Config};

/// CLI arguments.
#[derive(Parser)]
struct Args {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _args = Args::parse();

    let config = Config::load()?;
    let client = Client::new(&config).await?;
    client.run().await?;

    Ok(())
}
