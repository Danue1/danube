use super::Type;
use danubec_symbol::Symbol;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Path {
    pub segments: Vec<PathSegment>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PathSegment {
    pub ident: Symbol,
    pub type_arguments: Vec<Type>,
}

impl Path {
    pub const fn empty() -> Self {
        Self { segments: vec![] }
    }
}

impl std::ops::Add for Path {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        let mut segments = self.segments;
        segments.extend(rhs.segments);
        Self { segments }
    }
}
