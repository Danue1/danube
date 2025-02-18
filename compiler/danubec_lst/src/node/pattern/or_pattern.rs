ast_node! {
    /// ```ebnf
    /// OrPattern =
    /// | Pattern _ "|" _ OrPatternRhs
    /// ```
    struct OrPattern;

    node lhs -> Pattern;
    tokens pipe -> PIPE;
    node rhs -> OrPatternRhs;
}

ast_node! {
    /// ```ebnf
    /// OrPatternRhs =
    /// | Pattern
    /// ```
    struct OrPatternRhs;

    node pattern -> Pattern;
}
