use tabled::Tabled;

use sfd_core::result::IndexResult;

#[derive(Tabled)]
pub struct OkFileRow {
    #[tabled(rename = "File")]
    pub file: String,

    #[tabled(rename = "Items")]
    pub count: String,
}

#[derive(Tabled)]
pub struct BadFileRow {
    #[tabled(rename = "File")]
    pub file: String,

    #[tabled(rename = "Error")]
    pub error: String,
}

#[derive(Tabled)]
pub struct SkippedRow {
    #[tabled(rename = "File")]
    pub file: String,

    #[tabled(rename = "Reason")]
    pub reason: String,
}

#[derive(Tabled)]
pub struct TotalRow {
    #[tabled(rename = "")]
    pub label: String,

    #[tabled(rename = "Count")]
    pub count: String,
}

pub fn ok_rows(result: &IndexResult) -> Vec<OkFileRow> {
    let mut rows: Vec<_> = result
        .sources
        .iter()
        .map(|(source, items)| OkFileRow {
            file: source.path().display().to_string(),
            count: items.items.len().to_string(),
        })
        .collect();
    rows.sort_by(|a, b| a.file.cmp(&b.file));
    rows
}

pub fn bad_rows(result: &IndexResult) -> Vec<BadFileRow> {
    let mut rows: Vec<_> = result
        .errors
        .iter()
        .map(|(source, error)| BadFileRow {
            file: source.path().display().to_string(),
            error: error.to_string(),
        })
        .collect();
    rows.sort_by(|a, b| a.file.cmp(&b.file));
    rows
}

pub fn skipped_rows(result: &IndexResult) -> Vec<SkippedRow> {
    let mut rows: Vec<_> = result
        .skipped
        .iter()
        .map(|(path, reason)| SkippedRow {
            file: path.display().to_string(),
            reason: format!("{reason:?}"),
        })
        .collect();
    rows.sort_by(|a, b| a.file.cmp(&b.file));
    rows
}

pub fn total_rows(result: &IndexResult) -> Vec<TotalRow> {
    let items: usize = result.sources.values().map(|s| s.items.len()).sum();
    vec![
        TotalRow {
            label: "Items extracted:".to_string(),
            count: items.to_string(),
        },
        TotalRow {
            label: "Files processed:".to_string(),
            count: result.sources.len().to_string(),
        },
        TotalRow {
            label: "Files failed:".to_string(),
            count: result.errors.len().to_string(),
        },
        TotalRow {
            label: "Paths skipped:".to_string(),
            count: result.skipped.len().to_string(),
        },
    ]
}
