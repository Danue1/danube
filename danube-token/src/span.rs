#[derive(Debug, PartialEq, Clone)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Span {
    #[inline(always)]
    pub const fn new(start: usize, end: usize) -> Self {
        Span { start, end }
    }

    #[inline(always)]
    pub const fn concat(start: Span, end: Span) -> Self {
        Span {
            start: start.start,
            end: end.end,
        }
    }

    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        (self.end - self.start) == 0
    }
}
