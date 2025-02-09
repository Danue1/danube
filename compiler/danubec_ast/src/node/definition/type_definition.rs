ast_node! {
    /// ```ebnf
    /// TypeDefinition =
    /// | "type" _ Identifier _ "=" _ Type _ ";"
    /// | "type" _ Identifier _ "<" _ ">" _ "=" _ Type _ ";"
    /// | "type" _ Identifier _ "<" _ TypeParameter _ ">" _ "=" _ Type _ ";"
    /// | "type" _ Identifier _ "<" _ ( TypeParameter _ "," )+ _ ">" _ "=" _ Type _ ";"
    /// | "type" _ Identifier _ "<" _ ( TypeParameter _ "," )+ _ TypeParameter _ ">" _ "=" _ Type _ ";"
    /// ```
    struct TypeDefinition;

    token ty_token -> TYPE;
    node identifier -> Identifier;
    token left_chevron -> LEFT_CHEVRON;
    nodes type_parameter -> TypeParameter;
    token right_chevron -> RIGHT_CHEVRON;
    token equal -> EQUAL;
    node ty -> Type;
    token semicolon -> SEMICOLON;
}

ast_node! {
    /// ```ebnf
    /// TypeParameter =
    /// | Identifier
    /// | Identifier _ ":"
    /// | Identifier _ ":" _ Type
    /// | Identifier _ ":" ( _ Type _ "+" )+
    /// | Identifier _ ":" ( _ Type _ "+" )+ _ Type
    /// ```
    struct TypeParameter;

    node identifier -> Identifier;
    token colon -> COLON;
    node ty -> Type;
    token comma -> COMMA;
}
