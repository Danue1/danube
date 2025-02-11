ast_node! {
    /// ```ebnf
    /// UnaryExpression =
    /// | UnaryOperator Expression
    /// ```
    struct UnaryExpression;

    node operator -> UnaryOperator;
    node expression -> Expression;
}

ast_node! {
    /// ```ebnf
    /// UnaryOperator =
    /// | "-"
    /// | "!"
    /// | "~"
    /// ```
    struct UnaryOperator;

    token hyphen -> HYPHEN;
    token exclamation -> EXCLAMATION;
    token tilde -> TILDE;
}
