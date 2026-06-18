use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::Arc,
};

use crate::{extract::error::Error, models::lang_name::LangName};

#[derive(Debug, Clone)]
struct SourceInner {
    /// Source's path.
    ///
    /// This should be a **relative** path (relative to project dir).
    path: PathBuf,

    lang: Option<LangName>,

    content: String,
}

/// A source file.
#[derive(Debug, Clone)]
pub struct Source {
    inner: Arc<SourceInner>,
}

impl Source {
    pub async fn new(
        path: impl Into<PathBuf>,
        lang_exts: &HashMap<LangName, Vec<String>>,
    ) -> Result<Self, Error> {
        let path: PathBuf = path.into();
        let source = tokio::fs::read_to_string(&path).await?;
        let lang = guess_lang(&path, lang_exts);
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

    pub fn lang(&self) -> Option<&LangName> {
        self.inner.lang.as_ref()
    }

    pub fn content(&self) -> &str {
        &self.inner.content
    }
}

fn guess_lang(path: &Path, lang_exts: &HashMap<LangName, Vec<String>>) -> Option<LangName> {
    let ext = path.extension()?.to_str()?;
    lang_exts
        .iter()
        .find_map(|(name, exts)| exts.iter().any(|e| e == ext).then_some(name.clone()))
}
