use std::{path::PathBuf, sync::Arc};

use globset::{GlobSet, GlobSetBuilder};
use ignore::{Walk, WalkBuilder};

use crate::{
    config::spec::Config,
    models::{project_sources::ProjectSources, source::Source},
    scan::error::Error,
    util,
};

/// Scan context.
#[derive(Debug, Clone)]
struct ScanContext {
    inner: Arc<ScanContextInner>,
}

#[derive(Debug)]
struct ScanContextInner {
    root_path: PathBuf,
    exclude: GlobSet,
    ignore_git: bool,
    ignore_ignore: bool,
    ignore_hidden: bool,
}

impl ScanContext {
    fn new(config: &Config) -> Result<Self, Error> {
        let root_path = config.root().to_path_buf();

        let mut exclude_builder = GlobSetBuilder::new();
        for pattern in &config.scan.exclude {
            exclude_builder.add(globset::Glob::new(pattern)?);
        }
        let exclude = exclude_builder.build()?;

        Ok(Self {
            inner: Arc::new(ScanContextInner {
                root_path,
                exclude,
                ignore_git: config.scan.ignore_git,
                ignore_ignore: config.scan.ignore_ignore,
                ignore_hidden: config.scan.ignore_hidden,
            }),
        })
    }

    fn root(&self) -> &std::path::Path {
        &self.inner.root_path
    }
}

/// Scans the project.
pub async fn scan(config: &Config) -> Result<ProjectSources, Error> {
    let ctx = ScanContext::new(config)?;

    let root = ctx.root().to_path_buf();
    let entries = make_entries_iter(ctx)?;

    let mut srcs = Vec::new();
    for entry in entries {
        let Ok(entry) = entry else {
            // TODO: debug output
            continue;
        };

        if !entry.file_type().is_some_and(|t| t.is_file()) {
            continue;
        }

        let rel = util::to_rel(entry.path(), &root);
        let src = Source::new(rel, config).await?;

        srcs.push(src);
    }

    Ok(ProjectSources::new(srcs))
}

fn make_entries_iter(ctx: ScanContext) -> Result<Walk, Error> {
    let root = ctx.inner.root_path.clone();
    let exclude = ctx.inner.exclude.clone();

    let entries = WalkBuilder::new(&root)
        .standard_filters(false)
        .git_ignore(ctx.inner.ignore_git)
        .ignore(ctx.inner.ignore_ignore)
        .hidden(ctx.inner.ignore_hidden)
        .filter_entry(move |entry| {
            let rel = util::to_rel(entry.path(), &root);
            !exclude.is_match(rel)
        })
        .build();

    Ok(entries)
}
