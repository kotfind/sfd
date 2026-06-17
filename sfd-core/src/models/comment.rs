/// A comment from source.
#[derive(Debug, Clone)]
pub struct Comment {
    content: String,
}

impl Comment {
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}
