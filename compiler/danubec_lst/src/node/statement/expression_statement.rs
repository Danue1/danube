ast_node! {
    /// ```ebnf
    /// ExpressionStatement =
    /// | Expression
    /// | Expression _ ";"
    /// ```
    struct ExpressionStatement;

    node expression -> Expression;
    token semicolon -> SEMICOLON;
}
