mod cmd;
mod error;

use clap::{Parser, Subcommand};
use cmd::{index::IndexCmd, search::SearchCmd};
use sfd_core::{Client, config::Config};

use crate::error::Error;

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
    Index(IndexCmd),
    Search(SearchCmd),
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args = Args::parse();

    let config = Config::load()?;
    let allow_create_db = matches!(args.command, Command::Index(..));
    let client = Client::new(&config, allow_create_db).await?;

    match args.command {
        Command::Index(cmd) => cmd::index::run(cmd, client).await?,
        Command::Search(cmd) => cmd::search::run(cmd, client).await?,
    }

    Ok(())
}
