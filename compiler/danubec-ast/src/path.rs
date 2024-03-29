use crate::Ident;

#[derive(Debug, PartialEq)]
pub struct Path {
    pub segments: Vec<Ident>,
}

impl Path {
    pub const fn new(segments: Vec<Ident>) -> Self {
        Self { segments }
    }
}
