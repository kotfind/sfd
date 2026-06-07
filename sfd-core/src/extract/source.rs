use std::{path::{Path, PathBuf}, sync::Arc};

use crate::extract::error::Error;

#[derive(Debug, Clone)]
struct SourceInner {
    path: PathBuf,

    lines: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Source {
    inner: Arc<SourceInner>,
}

impl Source {
    pub async fn new(path: impl Into<PathBuf>) -> Result<Self, Error> {
        let path: PathBuf = path.into();
        let source = tokio::fs::read_to_string(&path).await?;
        let lines = source.lines().map(String::from).collect();
        Ok(Self {
            inner: Arc::new(SourceInner { path, lines }),
        })
    }

    pub fn path(&self) -> &Path {
        &self.inner.path
    }

    pub fn lines(&self) -> &[String] {
        &self.inner.lines
    }
}
