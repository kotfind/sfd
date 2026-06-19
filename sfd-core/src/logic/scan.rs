use ignore::{Walk, WalkBuilder};

use crate::{
    context::ScanContext,
    error::ScanError,
    models::{project_sources::ProjectSources, source::Source},
    util,
};

/// Scans the project.
pub(crate) async fn scan(ctx: ScanContext) -> Result<ProjectSources, ScanError> {
    let root = ctx.root().to_path_buf();
    let entries = make_entries_iter(ctx.clone())?;

    let mut srcs = Vec::new();
    for entry in entries {
        let Ok(entry) = entry else {
            continue;
        };

        if !entry.file_type().is_some_and(|t| t.is_file()) {
            continue;
        }

        let rel = util::to_rel(entry.path(), &root);
        let src = Source::new(rel);

        srcs.push(src);
    }

    Ok(ProjectSources::new(srcs))
}

fn make_entries_iter(ctx: ScanContext) -> Result<Walk, ScanError> {
    let root = ctx.root_path().clone();
    let exclude = ctx.exclude().clone();

    let entries = WalkBuilder::new(&root)
        .standard_filters(false)
        .git_ignore(ctx.ignore_git())
        .ignore(ctx.ignore_ignore())
        .hidden(ctx.ignore_hidden())
        .filter_entry(move |entry| {
            let rel = util::to_rel(entry.path(), &root);
            !exclude.is_match(rel)
        })
        .build();

    Ok(entries)
}
