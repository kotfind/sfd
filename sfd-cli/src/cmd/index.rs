use clap::Parser;
use sfd_core::Client;

use crate::{
    error::Error,
    view::{self, table},
};

/// Indexes the project.
#[derive(Parser)]
pub struct IndexCmd;

pub async fn run(_cmd: IndexCmd, client: Client) -> Result<(), Error> {
    let result = client.index().await?;

    table::print(
        view::table::index::ok_rows(&result),
        Some("Processed Files"),
    );

    if !result.errors.is_empty() {
        table::print(view::table::index::bad_rows(&result), Some("Failed Files"));
    }

    if !result.skipped.is_empty() {
        table::print(
            view::table::index::skipped_rows(&result),
            Some("Skipped Paths"),
        );
    }

    table::print(view::table::index::total_rows(&result), Some("Total"));

    Ok(())
}
