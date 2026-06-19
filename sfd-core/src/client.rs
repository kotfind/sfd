use std::collections::HashMap;

use time::UtcDateTime;

use crate::{
    SearchResult,
    config::Config,
    context::{DbContext, ExtractContext, ScanContext, VectContext},
    error::{Error, ExtractError},
    logic::{db, extract, guess_lang, ollama, scan},
    models::{skip_reason::SkipReason, source::Source},
    result::IndexResult,
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
    pub async fn index(&self) -> Result<IndexResult, Error> {
        let mut result = IndexResult {
            date: UtcDateTime::now(),
            sources: HashMap::new(),
            errors: HashMap::new(),
            skipped: HashMap::new(),
        };
        let project = scan::scan(self.scan.clone()).await?;

        for source in project.sources {
            self.process_source(source, &mut result).await?;
        }

        Ok(result)
    }

    async fn process_source(&self, source: Source, result: &mut IndexResult) -> Result<(), Error> {
        let lang_name = match guess_lang(source.path(), self.scan.ext_to_lang()) {
            Some(name) => name,
            None => {
                result
                    .skipped
                    .insert(source.path().to_path_buf(), SkipReason::NoLang);
                return Ok(());
            }
        };

        let source_items =
            match extract::extract(source.clone(), lang_name.clone(), &self.extract).await {
                Ok(items) => items,
                Err(ExtractError::File(e)) => {
                    result.errors.insert(source, e);
                    return Ok(());
                }
                Err(e) => return Err(e.into()),
            };

        let texts: Vec<&str> = source_items
            .items
            .iter()
            .map(|i| i.comment.content())
            .collect();
        let embeddings = ollama::embed(texts, self.vect.clone()).await?;

        db::insert_source(
            self.db.clone(),
            source_items.clone(),
            &embeddings,
            result.date,
        )
        .await?;

        result.sources.insert(source, source_items);

        Ok(())
    }

    /// Searches indexed comments.
    pub async fn search(&self, query: &str, limit: u32) -> Result<Vec<SearchResult>, Error> {
        let embedding = ollama::embed([query], self.vect.clone())
            .await?
            .into_iter()
            .next()
            .expect("expected exactly one embedding");

        Ok(db::search(self.db.clone(), &embedding, limit).await?)
    }
}
