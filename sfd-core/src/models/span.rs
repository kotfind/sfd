use crate::models::source::Source;

/// A position in a source file.
#[derive(Debug, Clone)]
pub struct Span {
    pub src: Source,

    /// Global offset (in chars).
    pub offset: usize,

    /// Line number (0-based).
    pub line: usize,

    /// Column number (0-based).
    pub col: usize,
}

impl Span {
    pub fn new(src: Source, offset: usize, line: usize, col: usize) -> Self {
        Self {
            src,
            offset,
            line,
            col,
        }
    }
}
