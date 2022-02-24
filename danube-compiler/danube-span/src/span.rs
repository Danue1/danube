use crate::Location;

#[derive(Debug, PartialEq, Clone)]
pub struct Span {
    pub start: Location,
    pub end: Location,
}

impl Span {
    pub const fn new(start: Location, end: Location) -> Self {
        Span { start, end }
    }

    pub const fn with_start(&self, start: Location) -> Self {
        Span {
            start,
            end: self.end,
        }
    }

    pub const fn with_end(&self, end: Location) -> Self {
        Span {
            start: self.start,
            end,
        }
    }
}
