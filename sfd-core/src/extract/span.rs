use crate::extract::source::Source;

#[derive(Debug, Clone)]
pub struct Span {
    pub src: Source,
    pub line: usize,
    pub col: usize,
}

impl Span {
    pub fn new(src: Source, line: usize, col: usize) -> Self {
        Self { src, line, col }
    }
}
