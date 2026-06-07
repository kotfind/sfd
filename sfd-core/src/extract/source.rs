use std::{path::{Path, PathBuf}, sync::Arc};

use crate::extract::error::Error;

#[derive(Debug, Clone)]
struct SourceInner {
    path: PathBuf,

    ext: Option<String>,

    content: String,
}

#[derive(Debug, Clone)]
pub struct Source {
    inner: Arc<SourceInner>,
}

impl Source {
    pub async fn new(path: impl Into<PathBuf>) -> Result<Self, Error> {
        let path: PathBuf = path.into();
        let source = tokio::fs::read_to_string(&path).await?;
        let ext = path.extension().and_then(|e| e.to_str().map(String::from));
        Ok(Self {
            inner: Arc::new(SourceInner { path, ext, content: source }),
        })
    }

    pub fn path(&self) -> &Path {
        &self.inner.path
    }

    pub fn ext(&self) -> Option<&str> {
        self.inner.ext.as_deref()
    }

    pub fn content(&self) -> &str {
        &self.inner.content
    }
}
