use super::Type;
use crate::HirId;
use danubec_symbol::Symbol;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Path {
    pub segments: Vec<PathSegment>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PathSegment {
    pub hir_id: HirId,
    pub ident: Symbol,
    pub type_arguments: Vec<Type>,
}
