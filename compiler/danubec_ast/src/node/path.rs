use super::Type;
use danubec_symbol::Symbol;
use danubec_syntax::SyntaxNode;

pub struct Path {
    pub syntax: SyntaxNode,
    pub segments: Vec<PathSegment>,
}

pub struct PathSegment {
    pub syntax: SyntaxNode,
    pub ident: Symbol,
    pub types: Vec<Type>,
}
