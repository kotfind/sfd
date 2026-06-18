use time::UtcDateTime;

use crate::{
    config::Config,
    context::{DbContext, ExtractContext, ScanContext, VectContext},
    error::Error,
    logic::{self, db, ollama},
};

/// App client.
///
/// Entry point to the library.
#[derive(Debug, Clone)]
pub struct Client {
    db: DbContext,
    vect: VectContext,
    extract: ExtractContext,
    scan: ScanContext,
}

impl Client {
    /// Creates a new client from config.
    pub async fn new(config: &Config) -> Result<Self, Error> {
        let db = DbContext::new(config).await?;
        let vect = VectContext::new(config)?;
        let extract = ExtractContext::new(config)?;
        let scan = ScanContext::new(config)?;

        Ok(Self {
            db,
            vect,
            extract,
            scan,
        })
    }

    /// Runs the full pipeline.
    pub async fn run(&self) -> Result<(), Error> {
        ollama::ping(self.vect.clone()).await?;
        if !ollama::has_model(self.vect.clone()).await? {
            ollama::pull_model(self.vect.clone()).await?;
        }

        let project = logic::scan::scan(self.scan.clone()).await?;

        for source in project.sources {
            let source_items = match logic::extract::extract(source, &self.extract) {
                Ok(items) => items,
                Err(e) => {
                    if e.is_file_local() {
                        eprintln!("extraction error: {e}");
                        continue;
                    }
                    return Err(Error::Extract(e));
                }
            };

            let mut embeddings = Vec::new();
            for item in &source_items.items {
                let embedding = ollama::embed(item.comment.content(), self.vect.clone()).await?;
                embeddings.push(embedding);
            }

            let now = UtcDateTime::now();
            db::insert_source(self.db.clone(), source_items, &embeddings, now).await?;
        }

        Ok(())
    }
}
