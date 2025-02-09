pub mod path_type;

pub use path_type::*;

ast_node! {
    /// ```ebnf
    /// Type =
    /// | PathType
    /// ```
    struct Type;

    node kind -> TypeKind;
}

ast_node! {
    /// ```ebnf
    /// TypeKind =
    /// | Path
    /// ```
    enum TypeKind {
        Path(PathType),
    }
}
