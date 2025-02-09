ast_node! {
    /// ```ebnf
    /// LetExpression =
    /// | "let" _ Identifier _ "=" _ Expression
    /// ```
    struct LetExpression;

    token let_token -> LET;
    node lhs -> Identifier;
    token equal -> EQUAL;
    node rhs -> Expression;
}
