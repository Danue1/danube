use super::Type;
use danubec_symbol::Symbol;

pub struct Path {
    pub segments: Vec<PathSegment>,
}

pub struct PathSegment {
    pub ident: Symbol,
    pub types: Vec<Type>,
}

impl Path {
    pub const fn empty() -> Self {
        Self { segments: vec![] }
    }
}
