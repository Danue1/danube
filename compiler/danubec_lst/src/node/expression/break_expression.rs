ast_node! {
    /// ```ebnf
    /// BreakExpression =
    /// | "break"
    /// ```
    struct BreakExpression;

    token break_token -> BREAK;
}
