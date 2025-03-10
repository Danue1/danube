ast_node! {
    /// ```ebnf
    /// ConstDefinition = "const" Identifier ":" Type "=" Expression ";"
    /// ```
    struct ConstDefinition;

    token const_token -> CONST;
    node identifier -> Identifier;
    token colon -> COLON;
    node ty -> Type;
    token equal -> EQUAL;
    node expression -> Expression;
    token semicolon -> SEMICOLON;
}
