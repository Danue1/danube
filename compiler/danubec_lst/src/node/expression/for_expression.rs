ast_node! {
    /// ```ebnf
    /// ForExpression =
    /// | "for" _ Pattern _ ForIterator _ BlockExpression
    /// ```
    struct ForExpression;

    token for_token -> FOR;
    node pattern -> Pattern;
    node iterator -> ForIterator;
    node block -> BlockExpression;
}

ast_node! {
    /// ```ebnf
    /// ForIterator =
    /// | "in" _ Expression
    /// ```
    struct ForIterator;

    token in_token -> IN;
    node expression -> Expression;
}
