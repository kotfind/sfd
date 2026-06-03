use crate::extract::source::Source;

#[derive(Debug, Clone)]
pub struct Span {
    pub src: Source,
    pub line: usize,
    pub col: usize,
}

impl Span {
    pub fn new(src: Source, line: usize, col: usize) -> Self {
        assert!(line < src.lines().len());
        assert!(col <= src.lines()[line].len());

        Self { src, line, col }
    }
}
