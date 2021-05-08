#[derive(Debug, PartialEq, Clone)]
pub struct Position {
    pub start: usize,
    pub end: usize,
}

impl Position {
    #[inline]
    pub const fn new(start: usize, end: usize) -> Self {
        Position { start, end }
    }

    #[inline]
    pub const fn concat(start: Position, end: Position) -> Self {
        Position {
            start: start.start,
            end: end.end,
        }
    }

    pub fn is_empty(&self) -> bool {
        (self.end - self.start) == 0
    }
}
