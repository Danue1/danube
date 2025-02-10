ast_node! {
    /// ```ebnf
    /// StructDefinition =
    /// | "struct" _ Identifier _ StructBodyKind
    /// | "struct" _ Identifier _ TypeParameters _ StructBodyKind
    /// | "struct" _ Identifier _ WhereClause _ StructBodyKind
    /// | "struct" _ Identifier _ TypeParameters _ WhereClause _ StructBodyKind
    /// ```
    struct StructDefinition;

    token struct_token -> STRUCT;
    node identifier -> Identifier;
    nodes type_parameters -> TypeParameter;
    node where_clause -> WhereClause;
    node body -> StructBodyKind;
}

ast_node! {
    /// ```ebnf
    /// StructBodyKind =
    /// | StructBodyNamed
    /// | StructBodyUnnamed
    /// ```
    enum StructBodyKind {
        Named(StructBodyNamed),
        Unnamed(StructBodyUnnamed),
    }
}

ast_node! {
    /// ```ebnf
    /// StructBodyNamed =
    /// "{" _ "}"
    /// "{" _ StructBodyNamedField _ "}"
    /// "{" _ ( StructBodyNamedField _ "," )+ _ "}"
    /// "{" _ ( StructBodyNamedField _ "," )+ _ StructBodyNamedField _ "}"
    /// ```
    struct StructBodyNamed;

    token left_brace -> LEFT_BRACE;
    nodes fields -> StructBodyNamedField;
    token right_brace -> RIGHT_BRACE;
}

ast_node! {
    /// ```ebnf
    /// StructBodyNamedField =
    /// | Visibility _ Identifier _ ":" _ Type
    /// | Visibility _ Identifier _ ":" _ Type _ ","
    /// ```
    struct StructBodyNamedField;

    node visibility -> Visibility;
    node identifier -> Identifier;
    token colon -> COLON;
    node ty -> Type;
    token semicolon -> SEMICOLON;
}

ast_node! {
    /// ```ebnf
    /// StructBodyUnnamed =
    /// "(" _ ")"
    /// "(" _ StructBodyUnnamedField _ ")"
    /// "(" _ ( StructBodyUnnamedField _ "," )+ _ ")"
    /// "(" _ ( StructBodyUnnamedField _ "," )+ _ StructBodyUnnamedField _ ")"
    /// ```
    struct StructBodyUnnamed;

    token left_paren -> LEFT_PAREN;
    nodes fields -> StructBodyUnnamedField;
    token right_paren -> RIGHT_PAREN;
}

ast_node! {
    /// ```ebnf
    /// StructBodyUnnamedField =
    /// | Type
    /// ```
    struct StructBodyUnnamedField;

    node ty -> Type;
    token comma -> COMMA;
}
