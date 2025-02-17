use super::Path;

pub struct Type {
    pub kind: TypeKind,
}

pub enum TypeKind {
    Never,
    Slice(Box<Type>),
    // Function(FunctionType),
    Path(Path),
    Tuple(Vec<Type>),
}
