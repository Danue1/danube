use super::Path;
use danubec_syntax::SyntaxNode;

pub struct Visibility {
    pub syntax: SyntaxNode,
    pub kind: VisibilityKind,
}

pub enum VisibilityKind {
    Crate,
    Super,
    In(Path),
}
