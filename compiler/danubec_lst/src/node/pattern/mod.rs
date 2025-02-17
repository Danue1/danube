pub mod array_pattern;
pub mod literal_pattern;
pub mod named_pattern;
pub mod never_pattern;
pub mod or_pattern;
pub mod path_pattern;
pub mod placeholder_pattern;
pub mod rest_pattern;
pub mod tuple_pattern;
pub mod unnamed_pattern;

pub use array_pattern::*;
pub use literal_pattern::*;
pub use named_pattern::*;
pub use never_pattern::*;
pub use or_pattern::*;
pub use path_pattern::*;
pub use placeholder_pattern::*;
pub use rest_pattern::*;
pub use tuple_pattern::*;
pub use unnamed_pattern::*;

ast_node! {
    /// ```ebnf
    /// Pattern =
    /// | PatternKind
    /// ```
    struct Pattern;

    node kind -> PatternKind;
}

ast_node! {
    /// ```ebnf
    /// PatternKind =
    /// | NeverPattern
    /// | PlaceholderPattern
    /// | PathPattern
    /// | TuplePattern
    /// | ArrayPattern
    /// | LiteralPattern
    /// | RestPattern
    /// | OrPattern
    /// | NamedPattern
    /// | UnnamedPattern
    /// ```
    enum PatternKind {
        Never(NeverPattern),
        Placeholder(PlaceholderPattern),
        Path(PathPattern),
        Tuple(TuplePattern),
        Array(ArrayPattern),
        Literal(LiteralPattern),
        Rest(RestPattern),
        Or(OrPattern),
        Named(NamedPattern),
        Unnamed(UnnamedPattern),
    }
}
