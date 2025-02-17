ast_node! {
    /// ```ebnf
    /// LetExpression =
    /// | "let" _ Pattern _ "=" _ Expression
    /// ```
    struct LetExpression;

    token let_token -> LET;
    node pattern -> Pattern;
    token equal -> EQUAL;
    node expression -> Expression;
}
