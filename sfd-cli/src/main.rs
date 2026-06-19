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

    /// Semantically searches the project.
    Search {
        /// Search query.
        query: String,

        /// Number of results.
        #[arg(short = 'n', long, default_value = "10")]
        number: u32,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let config = Config::load()?;
    let allow_create_db = matches!(args.command, Command::Index);
    let client = Client::new(&config, allow_create_db).await?;

    match args.command {
        Command::Index => client.index().await?,
        Command::Search { query, number } => {
            let results = client.search(&query, number).await?;
            for r in results {
                let sim = r.sim * 100.0;
                println!(
                    "{}:{}:{} ({:.0}%) {}",
                    r.loc.src.path().display(),
                    r.loc.line_num + 1,
                    r.loc.col_num + 1,
                    sim,
                    r.text,
                );
            }
        }
    }

    Ok(())
}
