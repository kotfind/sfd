use crate::extract::span::Span;

#[derive(Debug, Clone)]
pub struct CommentBlock {
    pub span: Span,
    content: String,
}

impl CommentBlock {
    pub fn new(span: Span, content: impl Into<String>) -> Self {
        Self {
            span,
            content: content.into(),
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}
