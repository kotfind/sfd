use std::path::Path;

use globset::{GlobSet, GlobSetBuilder};
use ignore::{Walk, WalkBuilder};

use crate::{
    config::spec::Config,
    models::{project_sources::ProjectSources, source::Source},
    scan::error::Error,
    util,
};

pub async fn scan_project(config: &Config) -> Result<ProjectSources, Error> {
    let root = get_root(config);

    let entries = make_entries_iter(config)?;

    let mut srcs = Vec::new();
    for entry in entries {
        let Ok(entry) = entry else {
            // TODO: debug output
            continue;
        };

        let rel = util::to_rel(entry.path(), root);
        let src = Source::new(rel, config).await?;

        srcs.push(src);
    }

    Ok(ProjectSources::new(srcs))
}

fn make_entries_iter(config: &Config) -> Result<Walk, Error> {
    let root = get_root(config).to_path_buf();

    let exclude = make_exclude_glob(config)?;

    let entries = WalkBuilder::new(&root)
        .standard_filters(false)
        .git_ignore(config.scan.ignore_git)
        .ignore(config.scan.ignore_ignore)
        .hidden(config.scan.ignore_hidden)
        .filter_entry(move |entry| {
            let rel = util::to_rel(entry.path(), &root);
            !exclude.is_match(rel)
        })
        .build();

    Ok(entries)
}

fn make_exclude_glob(config: &Config) -> Result<GlobSet, Error> {
    let mut exclude_builder = GlobSetBuilder::new();

    for pattern in &config.scan.exclude {
        exclude_builder.add(globset::Glob::new(pattern)?);
    }

    let exclude = exclude_builder.build()?;
    Ok(exclude)
}

fn get_root(config: &Config) -> &Path {
    config
        .root_path
        .as_ref()
        .expect("root path not set in config")
}
