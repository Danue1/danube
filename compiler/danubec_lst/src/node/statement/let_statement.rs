ast_node! {
    /// ```ebnf
    /// LetStatement =
    /// | "let" _ Pattern _ ";"
    /// | "let" _ Pattern _ "=" _ Expression _ ";"
    /// | "let" _ Pattern _ ":" _ Type _ ";"
    /// | "let" _ Pattern _ ":" _ Type _ "=" _ Expression _ ";"
    /// ```
    struct LetStatement;

    token let_token -> LET;
    node pattern -> Pattern;
    token colon -> COLON;
    node ty -> Type;
    token equal -> EQUAL;
    node expression -> Expression;
    token semicolon -> SEMICOLON;
}
