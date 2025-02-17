ast_node! {
    /// ```ebnf
    /// LiteralExpression =
    /// | Literal
    /// ```
    struct LiteralExpression;

    node literal -> Literal;
}
