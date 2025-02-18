ast_node! {
    /// ```ebnf
    /// ContinueExpression =
    /// | "continue"
    /// ```
    struct ContinueExpression;

    token continue_token -> CONTINUE;
}
