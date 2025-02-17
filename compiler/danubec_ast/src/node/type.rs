use super::Path;

pub struct Type {
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
