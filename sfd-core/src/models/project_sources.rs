use crate::models::source::Source;

/// All the project's [`Source`]s.
#[derive(Debug, Clone)]
pub struct ProjectSources {
    pub sources: Vec<Source>,
}

impl ProjectSources {
    pub fn new(sources: impl IntoIterator<Item = Source>) -> Self {
        Self {
            sources: sources.into_iter().collect(),
        }
    }
}
