use super::Type;
use danubec_symbol::Symbol;

#[derive(Clone)]
pub struct Path {
    pub segments: Vec<PathSegment>,
}

#[derive(Clone)]
pub struct PathSegment {
    pub ident: Symbol,
    pub types: Vec<Type>,
}

impl Path {
    pub const fn empty() -> Self {
        Self { segments: vec![] }
    }
}
