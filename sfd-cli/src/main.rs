use clap::{Parser, Subcommand};
use sfd_core::{Client, config::Config};

/// Semantic Find.
///
/// Semantic search over codebase comments.
#[derive(Parser)]
#[command(version, about, disable_help_subcommand = true)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
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
