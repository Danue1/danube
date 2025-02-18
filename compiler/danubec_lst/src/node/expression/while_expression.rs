ast_node! {
    /// ```ebnf
    /// WhileExpression =
    /// | "while" _ Expression _ BlockExpression
    /// ```
    struct WhileExpression;

    token while_token -> WHILE;
    node expression -> Expression;
    node block -> BlockExpression;
}
