pub mod path_type;

pub use path_type::*;

ast_node! {
    struct Type;

    node kind -> TypeKind;
}

ast_node! {
    enum TypeKind {
        Path(PathType),
    }
}
