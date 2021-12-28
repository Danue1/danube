#[derive(Debug, PartialEq, Clone)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Span {
    pub const fn new(start: usize, end: usize) -> Self {
        Span { start, end }
    }

    pub const fn with_start(&self, start: usize) -> Self {
        Span {
            start,
            end: self.end,
        }
    }

    pub const fn with_end(&self, end: usize) -> Self {
        Span {
            start: self.start,
            end,
        }
    }

    pub const fn concat(&self, span: Span) -> Self {
        Span {
            start: self.start,
            end: span.end,
        }
    }
}
