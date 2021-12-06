#[derive(Debug, PartialEq, Clone)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Span {
    pub const fn new(start: usize, end: usize) -> Self {
        Span { start, end }
    }

    pub const fn concat(Span { start, .. }: Span, Span { end, .. }: Span) -> Self {
        Span { start, end }
    }

    pub const fn is_empty(&self) -> bool {
        (self.end - self.start) == 0
    }
}
