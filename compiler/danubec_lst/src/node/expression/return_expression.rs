ast_node! {
    /// ```ebnf
    /// ReturnExpression =
    /// | "return" _ Expression
    /// ```
    struct ReturnExpression;

    token return_token -> RETURN;
    node expression -> Expression;
}
