use std::{collections::HashMap, path::PathBuf, sync::Arc};

use globset::{GlobSet, GlobSetBuilder};

use crate::{config::Config, error::ScanError, models::lang_name::LangName};

/// Scan context.
#[derive(Debug, Clone)]
pub struct ScanContext {
    inner: Arc<ScanContextInner>,
}

#[derive(Debug)]
struct ScanContextInner {
    root_path: PathBuf,
    exclude: GlobSet,
    ignore_git: bool,
    ignore_ignore: bool,
    ignore_hidden: bool,
    ext_to_lang: HashMap<String, LangName>,
}

impl ScanContext {
    pub fn new(config: &Config) -> Result<Self, ScanError> {
        let root_path = config.root().to_path_buf();

        let mut exclude_builder = GlobSetBuilder::new();
        for pattern in &config.scan.exclude {
            exclude_builder.add(globset::Glob::new(pattern)?);
        }
        let exclude = exclude_builder.build()?;

        let mut ext_to_lang = HashMap::new();
        for (name, cfg) in &config.langs {
            for ext in &cfg.exts {
                ext_to_lang.insert(ext.clone(), name.clone());
            }
        }

        Ok(Self {
            inner: Arc::new(ScanContextInner {
                root_path,
                exclude,
                ignore_git: config.scan.ignore_git,
                ignore_ignore: config.scan.ignore_ignore,
                ignore_hidden: config.scan.ignore_hidden,
                ext_to_lang,
            }),
        })
    }

    pub fn root(&self) -> &std::path::Path {
        &self.inner.root_path
    }

    pub fn ext_to_lang(&self) -> &HashMap<String, LangName> {
        &self.inner.ext_to_lang
    }

    pub fn ignore_git(&self) -> bool {
        self.inner.ignore_git
    }

    pub fn ignore_ignore(&self) -> bool {
        self.inner.ignore_ignore
    }

    pub fn ignore_hidden(&self) -> bool {
        self.inner.ignore_hidden
    }

    pub fn root_path(&self) -> &PathBuf {
        &self.inner.root_path
    }

    pub fn exclude(&self) -> &GlobSet {
        &self.inner.exclude
    }
}
