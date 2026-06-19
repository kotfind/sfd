use clap::Parser;
use sfd_core::Client;

use crate::error::Error;

/// Semantically searches the project.
#[derive(Parser)]
pub struct SearchCmd {
    /// Search query.
    pub query: String,

    /// Number of results.
    #[arg(short = 'n', long, default_value = "10")]
    pub number: u32,
}

pub async fn run(cmd: SearchCmd, client: Client) -> Result<(), Error> {
    let results = client.search(&cmd.query, cmd.number).await?;
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

    Ok(())
}
