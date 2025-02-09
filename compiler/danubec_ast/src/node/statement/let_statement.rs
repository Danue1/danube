ast_node! {
    /// ```ebnf
    /// LetStatement =
    /// | "let" _ Identifier _ ";"
    /// | "let" _ Identifier _ "=" _ Expression _ ";"
    /// | "let" _ Identifier _ ":" _ Type _ ";"
    /// | "let" _ Identifier _ ":" _ Type _ "=" _ Expression _ ";"
    /// ```
    struct LetStatement;

    token let_token -> LET;
    node lhs -> Identifier;
    token colon -> COLON;
    node ty -> Type;
    token equal -> EQUAL;
    node rhs -> Expression;
    token semicolon -> SEMICOLON;
}
