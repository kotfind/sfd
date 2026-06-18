use std::{collections::HashMap, path::PathBuf, sync::Arc};

use globset::{GlobSet, GlobSetBuilder};

use crate::{config::spec::Config, models::lang_name::LangName, scan::error::Error};

/// Scan context.
#[derive(Debug, Clone)]
pub(crate) struct ScanContext {
    pub(crate) inner: Arc<ScanContextInner>,
}

#[derive(Debug)]
pub(crate) struct ScanContextInner {
    pub(crate) root_path: PathBuf,
    pub(crate) exclude: GlobSet,
    pub(crate) ignore_git: bool,
    pub(crate) ignore_ignore: bool,
    pub(crate) ignore_hidden: bool,
    pub(crate) lang_exts: HashMap<LangName, Vec<String>>,
}

impl ScanContext {
    pub(crate) fn new(config: &Config) -> Result<Self, Error> {
        let root_path = config.root().to_path_buf();

        let mut exclude_builder = GlobSetBuilder::new();
        for pattern in &config.scan.exclude {
            exclude_builder.add(globset::Glob::new(pattern)?);
        }
        let exclude = exclude_builder.build()?;

        let lang_exts = config
            .langs
            .iter()
            .map(|(name, cfg)| (name.clone(), cfg.exts.clone()))
            .collect();

        Ok(Self {
            inner: Arc::new(ScanContextInner {
                root_path,
                exclude,
                ignore_git: config.scan.ignore_git,
                ignore_ignore: config.scan.ignore_ignore,
                ignore_hidden: config.scan.ignore_hidden,
                lang_exts,
            }),
        })
    }

    pub(crate) fn root(&self) -> &std::path::Path {
        &self.inner.root_path
    }
}
