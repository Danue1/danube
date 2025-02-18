ast_node! {
    /// ```ebnf
    /// TryExpression =
    /// | Expression _ "?"
    /// ```
    struct TryExpression;

    node expression -> Expression;
    token question -> QUESTION;
}
