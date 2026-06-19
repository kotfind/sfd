use clap::Parser;
use sfd_core::Client;

use crate::{
    error::Error,
    view::table::{self, search::SearchRow},
};

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
    let rows: Vec<SearchRow> = results.into_iter().map(SearchRow::from).collect();
    table::print(rows);

    Ok(())
}
