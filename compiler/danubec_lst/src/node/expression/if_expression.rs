ast_node! {
    /// ```ebnf
    /// IfExpression =
    /// | "if" _ Expression _ BlockExpression
    /// | "if" _ Expression _ BlockExpression _ ElseBranch
    /// ```
    struct IfExpression;

    token if_token -> IF;
    node condition -> Expression;
    node then_branch -> BlockExpression;
    node else_branch -> ElseBranch;
}

ast_node! {
    /// ```ebnf
    /// ElseBranch =
    /// | "else" _ IfExpression
    /// | "else" _ BlockExpression
    /// ```
    struct ElseBranch;

    token else_token -> ELSE;
    node expression -> Expression;
    node block -> BlockExpression;
}
