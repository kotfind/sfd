use reqwest::Client;
use std::time::Duration;

use crate::{
    config::spec::Config, db, error::Error, extract, extract::state::State, scan::scanner, vect,
};

pub async fn run(config: &Config) -> Result<(), Error> {
    let pool = db::connect(config).await?;

    let client = Client::builder()
        .timeout(Duration::from_secs(config.vect.ollama.timeout))
        .build()?;

    let state = State::new(config)?;
    let project = scanner::scan_project(config).await?;

    for source in project.sources {
        let source_items = extract::extract_items(source, &state)?;

        for item in source_items.items {
            let embedding = vect::embed(item.comment.content(), config, &client).await?;

            // TODO: store item + embedding in db
        }
    }

    Ok(())
}
