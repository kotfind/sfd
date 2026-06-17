use clap::Parser;
use sfd::{config::Config, run};

/// CLI arguments.
#[derive(Parser)]
struct Args {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _args = Args::parse();

    let config = Config::load()?;
    run(&config).await?;

    Ok(())
}
