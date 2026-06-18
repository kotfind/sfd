use crate::{
    config::spec::Config,
    db,
    error::Error,
    extract,
    extract::state::State,
    scan::scanner,
    vect::{self, State as VectState},
};

/// Runs the whole pipeline.
pub async fn run(config: &Config) -> Result<(), Error> {
    let pool = db::connect(config).await?;

    let vect_state = VectState::new(config)?;
    vect::ollama::ping(vect_state.clone()).await?;
    if !vect::ollama::has_model(vect_state.clone()).await? {
        vect::ollama::pull_model(vect_state.clone()).await?;
    }

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
            let embedding = vect::ollama::embed(item.comment.content(), vect_state.clone()).await?;

            // TODO: store item + embedding in db
        }
    }

    Ok(())
}
