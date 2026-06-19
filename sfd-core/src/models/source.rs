use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

/// A source file.
#[derive(Debug, Clone)]
pub struct Source {
    path: Arc<PathBuf>,
}

impl Source {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self {
            path: Arc::new(path.into()),
        }
    }

    pub fn path(&self) -> &Path {
        &self.path
    }
}
