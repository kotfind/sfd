use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use crate::{config::spec::Config, extract::error::Error};

#[derive(Debug, Clone)]
struct SourceInner {
    /// Source's path.
    ///
    /// This should be a **relative** path (relative to project dir).
    path: PathBuf,

    lang: Option<String>,

    content: String,
}

/// A source file.
#[derive(Debug, Clone)]
pub struct Source {
    inner: Arc<SourceInner>,
}

impl Source {
    pub async fn new(path: impl Into<PathBuf>, config: &Config) -> Result<Self, Error> {
        let path: PathBuf = path.into();
        let source = tokio::fs::read_to_string(&path).await?;
        let lang = guess_lang(&path, config);
        Ok(Self {
            inner: Arc::new(SourceInner {
                path,
                lang,
                content: source,
            }),
        })
    }

    pub fn path(&self) -> &Path {
        &self.inner.path
    }

    pub fn lang(&self) -> Option<&str> {
        self.inner.lang.as_deref()
    }

    pub fn content(&self) -> &str {
        &self.inner.content
    }
}

fn guess_lang(path: &Path, config: &Config) -> Option<String> {
    let ext = path.extension()?.to_str()?;
    config.langs.iter().find_map(|(name, lang_cfg)| {
        lang_cfg
            .exts
            .iter()
            .any(|e| e == ext)
            .then_some(name.clone())
    })
}
