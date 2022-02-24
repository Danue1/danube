#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Location {
    pub offset: usize,
    pub line: usize,
    pub column: usize,
}

impl Location {
    #[inline]
    pub fn increment(&mut self) {
        self.offset += 1;
        self.column += 1;
    }

    #[inline]
    pub fn increment_line(&mut self) {
        self.offset += 1;
        self.line += 1;
        self.column = 1;
    }

    #[inline]
    pub fn decrement(&mut self) {
        self.offset -= 1;
        self.column -= 1;
    }
}
