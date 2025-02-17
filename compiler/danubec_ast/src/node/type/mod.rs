use super::Path;
use danubec_syntax::SyntaxNode;

pub struct Type {
    pub syntax: SyntaxNode,
    pub kind: TypeKind,
}

pub enum TypeKind {
    // Array(ArrayType),
    // Function(FunctionType),
    // Generic(GenericType),
    Path(Path),
    // Pointer(PointerType),
    // Tuple(TupleType),
}
