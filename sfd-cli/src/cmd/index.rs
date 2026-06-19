use clap::Parser;
use sfd_core::Client;

use crate::error::Error;

/// Indexes the project.
#[derive(Parser)]
pub struct IndexCmd;

pub async fn run(_cmd: IndexCmd, client: Client) -> Result<(), Error> {
    // TODO: display result
    client.index().await?;

    Ok(())
}
