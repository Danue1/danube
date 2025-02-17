pub mod never_type;
pub mod path_type;
pub mod slice_type;
pub mod tuple_type;

pub use never_type::*;
pub use path_type::*;
pub use slice_type::*;
pub use tuple_type::*;

ast_node! {
    /// ```ebnf
    /// Type =
    /// | TypeKind
    /// ```
    struct Type;

    node kind -> TypeKind;
}

ast_node! {
    /// ```ebnf
    /// TypeKind =
    /// | NeverType
    /// | PathType
    /// | SliceType
    /// | TupleType
    /// ```
    enum TypeKind {
        Never(NeverType),
        Path(PathType),
        Slice(SliceType),
        Tuple(TupleType),
    }
}
