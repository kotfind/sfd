use crate::{
    config::spec::Config,
    db,
    error::Error,
    extract::{self, ExtractContext},
    scan::scanner,
    vect::{self, VectContext},
};

/// Runs the whole pipeline.
pub async fn run(config: &Config) -> Result<(), Error> {
    let db_ctx = db::DbContext::new(config).await?;

    let vect_ctx = VectContext::new(config)?;
    vect::ollama::ping(vect_ctx.clone()).await?;
    if !vect::ollama::has_model(vect_ctx.clone()).await? {
        vect::ollama::pull_model(vect_ctx.clone()).await?;
    }

    let extract_ctx = ExtractContext::new(config)?;
    let project = scanner::scan(config).await?;

    for source in project.sources {
        let source_items = match extract::extract(source, &extract_ctx) {
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
            let embedding = vect::ollama::embed(item.comment.content(), vect_ctx.clone()).await?;

            // TODO: store item + embedding in db
        }
    }

    Ok(())
}
