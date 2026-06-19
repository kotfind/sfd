use crate::models::source::Source;

/// A position in a source file.
#[derive(Debug, Clone)]
pub struct Location {
    pub src: Source,

    /// Global offset (in chars).
    pub offset: usize,

    /// Line number (0-based).
    pub line_num: usize,

    /// Column number (0-based).
    pub col_num: usize,
}

impl Location {
    pub fn new(src: Source, offset: usize, line_num: usize, col_num: usize) -> Self {
        Self {
            src,
            offset,
            line_num,
            col_num,
        }
    }
}
