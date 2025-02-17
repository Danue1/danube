ast_node! {
    /// ```ebnf
    /// ExpressionStatement =
    /// | Expression
    /// ```
    struct ExpressionStatement;

    node expression -> Expression;
}
