use std::{collections::HashMap, sync::Arc};

use futures::stream::{FuturesUnordered, StreamExt};
use time::UtcDateTime;
use tokio::sync::Semaphore;

use crate::{
    SearchResult,
    config::Config,
    context::{DbContext, ExtractContext, ScanContext, VectContext},
    error::{Error, ExtractError, FileExtractError},
    logic::{db, extract, guess_lang, ollama, scan},
    models::{skip_reason::SkipReason, source::Source, source_items::SourceItems},
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
        let date = UtcDateTime::now();
        let mut result = IndexResult {
            date,
            sources: HashMap::new(),
            errors: HashMap::new(),
            skipped: HashMap::new(),
        };
        let project = scan::scan(self.scan.clone()).await?;

        let ollama_sem = Arc::new(Semaphore::new(self.vect.max_parallel()));

        let mut tasks: FuturesUnordered<_> = project
            .sources
            .into_iter()
            .map(|source| {
                let client = self.clone();
                let ollama_sem = ollama_sem.clone();
                async move { client.process_source(source, date, &ollama_sem).await }
            })
            .collect();

        while let Some(task_result) = tasks.next().await {
            match task_result? {
                SourceResult::Ok { source, items } => {
                    result.sources.insert(source, items);
                }
                SourceResult::Error { source, error } => {
                    result.errors.insert(source, error);
                }
                SourceResult::Skipped { path, reason } => {
                    result.skipped.insert(path, reason);
                }
            }
        }

        Ok(result)
    }

    async fn process_source(
        &self,
        source: Source,
        date: UtcDateTime,
        ollama_sem: &Semaphore,
    ) -> Result<SourceResult, Error> {
        let lang_name = match guess_lang(source.path(), self.scan.ext_to_lang()) {
            Some(name) => name,
            None => {
                return Ok(SourceResult::Skipped {
                    path: source.path().to_path_buf(),
                    reason: SkipReason::NoLang,
                });
            }
        };

        let source_clone = source.clone();
        let source_items =
            match extract::extract(source_clone, lang_name.clone(), &self.extract).await {
                Ok(items) => items,
                Err(ExtractError::File(e)) => return Ok(SourceResult::Error { source, error: e }),
                Err(e) => return Err(e.into()),
            };

        let texts: Vec<&str> = source_items
            .items
            .iter()
            .map(|i| i.comment.content())
            .collect();
        let embeddings = {
            let _permit = ollama_sem.acquire().await.expect("semaphore closed");
            ollama::embed(texts, self.vect.clone()).await?
        };

        db::insert_source(self.db.clone(), source_items.clone(), &embeddings, date).await?;

        Ok(SourceResult::Ok {
            source,
            items: source_items,
        })
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

enum SourceResult {
    Ok {
        source: Source,
        items: SourceItems,
    },
    Error {
        source: Source,
        error: FileExtractError,
    },
    Skipped {
        path: std::path::PathBuf,
        reason: SkipReason,
    },
}
