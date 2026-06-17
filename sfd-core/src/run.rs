use reqwest::Client;
use std::time::Duration;

use crate::{
    config::spec::Config, db, error::Error, extract, extract::state::State, scan::scanner, vect,
};

/// Runs the whole pipeline.
pub async fn run(config: &Config) -> Result<(), Error> {
    let pool = db::connect(config).await?;

    let client = Client::builder()
        .timeout(Duration::from_secs_f64(config.vect.ollama.timeout))
        .build()?;

    let state = State::new(config)?;
    let project = scanner::scan(config).await?;

    for source in project.sources {
        let source_items = match extract::extract(source, &state) {
            Ok(items) => items,
            Err(e) => {
                if e.is_file_local() {
                    eprintln!("extraction error: {e}");
                    continue;
                }
                return Err(Error::Extract(e));
            }
        };

        for item in source_items.items {
            let embedding = vect::embed(item.comment.content(), config, &client).await?;

            // TODO: store item + embedding in db
        }
    }

    Ok(())
}
