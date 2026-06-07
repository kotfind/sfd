use crate::models::source::Source;

#[derive(Debug, Clone)]
pub struct Span {
    pub src: Source,

    pub offset: usize,

    pub line: usize,

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
