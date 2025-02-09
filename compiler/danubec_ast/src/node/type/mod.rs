pub mod path_type;

pub use path_type::*;

ast_node! {
    /// ```
    /// Type =
    /// | PathType
    /// ```
    struct Type;

    node kind -> TypeKind;
}

ast_node! {
    /// ```
    /// TypeKind =
    /// | Path
    /// ```
    enum TypeKind {
        Path(PathType),
    }
}
