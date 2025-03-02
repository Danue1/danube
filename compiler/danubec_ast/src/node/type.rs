use super::Path;

#[derive(Clone)]
pub struct Type {
    pub kind: TypeKind,
}

#[derive(Clone)]
pub enum TypeKind {
    Never,
    Slice(Box<Type>),
    // Function(FunctionType),
    Path(Path),
    Tuple(Vec<Type>),
}
