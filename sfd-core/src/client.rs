use time::UtcDateTime;

use crate::{
    config::Config,
    context::{DbContext, ExtractContext, ScanContext, VectContext},
    error::Error,
    logic::{db, extract, guess_lang, ollama, scan},
    models::source::Source,
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
    pub async fn new(config: &Config, allow_create_db: bool) -> Result<Self, Error> {
        let db = db::connect(config, allow_create_db).await?;
        let vect = VectContext::new(config).await?;
        let extract = ExtractContext::new(config)?;
        let scan = ScanContext::new(config)?;

        Ok(Self {
            db,
            vect,
            extract,
            scan,
        })
    }

    /// Indexes the project.
    pub async fn index(&self) -> Result<(), Error> {
        let project = scan::scan(self.scan.clone()).await?;

        for source in project.sources {
            self.process_source(source).await?;
        }

        Ok(())
    }

    async fn process_source(&self, source: Source) -> Result<(), Error> {
        let lang_name = match guess_lang(source.path(), self.scan.ext_to_lang()) {
            Some(name) => name,
            None => {
                eprintln!(
                    "extraction error: no language detected for `{}`",
                    source.path().display()
                );
                return Ok(());
            }
        };

        let source_items = match extract::extract(source, &lang_name, &self.extract).await {
            Ok(items) => items,
            Err(e) => {
                if e.is_file_local() {
                    eprintln!("extraction error: {e}");
                    return Ok(());
                }
                return Err(Error::Extract(e));
            }
        };

        let texts: Vec<&str> = source_items
            .items
            .iter()
            .map(|i| i.comment.content())
            .collect();
        let embeddings = ollama::embed(texts, self.vect.clone()).await?;

        let now = UtcDateTime::now();
        db::insert_source(self.db.clone(), source_items, &embeddings, now).await?;

        Ok(())
    }

    /// Searches indexed comments.
    pub async fn search(&self, query: &str, limit: u32) -> Result<Vec<db::SearchResult>, Error> {
        let embedding = ollama::embed([query], self.vect.clone())
            .await?
            .into_iter()
            .next()
            .expect("expected exactly one embedding");

        Ok(db::search(self.db.clone(), &embedding, limit).await?)
    }
}
