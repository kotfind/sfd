use tabled::Tabled;

use sfd_core::SearchResult;

#[derive(Tabled)]
pub struct SearchRow {
    #[tabled(rename = "File")]
    pub file: String,

    #[tabled(rename = "Line")]
    pub line_num: String,

    #[tabled(rename = "Column")]
    pub col_num: String,

    #[tabled(rename = "Sim")]
    pub sim: String,

    #[tabled(rename = "Text")]
    pub text: String,
}

impl From<SearchResult> for SearchRow {
    fn from(r: SearchResult) -> Self {
        Self {
            file: r.loc.src.path().display().to_string(),
            line_num: (r.loc.line_num + 1).to_string(),
            col_num: (r.loc.col_num + 1).to_string(),
            sim: format!("{:.0}%", r.sim * 100.0),
            text: r.text,
        }
    }
}
