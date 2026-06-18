use clap::Parser;
use sfd_core::{Client, config::Config};

/// CLI arguments.
#[derive(Parser)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Parser)]
enum Command {
    /// Indexes the project.
    Index,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let config = Config::load()?;
    let client = Client::new(&config).await?;

    match args.command {
        Command::Index => client.index().await?,
    }

    Ok(())
}
