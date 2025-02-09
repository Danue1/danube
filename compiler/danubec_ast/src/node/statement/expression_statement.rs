ast_node! {
    /// ```
    /// ExpressionStatement =
    /// | Expression
    /// ```
    struct ExpressionStatement;

    node expression -> Expression;
}
