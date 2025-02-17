ast_node! {
    /// ```ebnf
    /// Path =
    /// | PathSegment
    /// | ( PathSegment _ "::" )+ PathSegment
    /// ```
    struct Path;

    nodes segments -> PathSegment;
}

ast_node! {
    /// ```ebnf
    /// PathSegment =
    /// | Identifier
    /// | Identifier _ TypeArguments
    struct PathSegment;

    node identifier -> Identifier;
    node type_argument -> TypeArgument;
}
