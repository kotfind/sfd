use crate::{
    config::spec::Config,
    db::DbContext,
    error::Error,
    extract::{self, ExtractContext},
    scan::{self, context::ScanContext},
    vect::{self, VectContext},
};

/// App context — owns all sub-contexts and runs the pipeline.
#[derive(Debug, Clone)]
pub struct Context {
    db: DbContext,
    vect: VectContext,
    extract: ExtractContext,
    scan: ScanContext,
}

impl Context {
    /// Creates a new context from config.
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
        vect::ollama::ping(self.vect.clone()).await?;
        if !vect::ollama::has_model(self.vect.clone()).await? {
            vect::ollama::pull_model(self.vect.clone()).await?;
        }

        let project = scan::scanner::scan(self.scan.clone()).await?;

        for source in project.sources {
            let source_items = match extract::extract(source, &self.extract) {
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
                let embedding = vect::embed(item.comment.content(), self.vect.clone()).await?;

                // TODO: store item + embedding in db
            }
        }

        Ok(())
    }
}
