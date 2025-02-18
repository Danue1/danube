ast_node! {
    /// ```ebnf
    /// LoopExpression =
    /// | "loop" _ BlockExpression
    /// ```
    struct LoopExpression;

    token loop_token -> LOOP;
    node block -> BlockExpression;
}
