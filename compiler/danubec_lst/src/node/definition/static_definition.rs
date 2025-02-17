ast_node! {
    /// ```ebnf
    /// StaticDefinition =
    /// | "static" _ Identifier _ ":" _ Type _ "=" _ Expression _ ";"
    /// ```
    struct StaticDefinition;

    token static_token -> STATIC;
    node identifier -> Identifier;
    token colon -> COLON;
    node ty -> Type;
    token equal -> EQUAL;
    node expression -> Expression;
    token semicolon -> SEMICOLON;
}
