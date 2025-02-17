ast_node! {
    /// ```ebnf
    /// UseDefinition =
    /// | "use" _ UseTree _ ";"
    /// | "use" _ "::" _ UseTree _ ";"
    /// ```
    struct UseDefinition;

    token use_token -> USE;
    token colon_colon -> COLON__COLON;
    node tree -> UseTree;
    token semicolon -> SEMICOLON;
}

ast_node! {
    /// ```ebnf
    /// UseTree =
    /// | Identifier _ UseTreeKind
    /// | "::" _ Identifier _ UseTreeKind
    /// ```
    struct UseTree;

    token colon_colon -> COLON__COLON;
    node path -> Path;
    node kind -> UseTreeKind;
}

ast_node! {
    /// ```ebnf
    /// UseTreeKind =
    /// | UseTreeBarrel
    /// | UseTreeIdent
    /// | UseTreeNested
    /// ```
    enum UseTreeKind {
        Barrel(UseTreeBarrel),
        Ident(UseTreeIdent),
        Nested(UseTreeNested),
    }
}

ast_node! {
    /// ```ebnf
    /// UseTreeBarrel =
    /// | "*"
    /// ```
    struct UseTreeBarrel;

    token asterisk -> ASTERISK;
}

ast_node! {
    /// ```ebnf
    /// UseTreeIdent =
    /// | "as" _ Identifier
    /// ```
    struct UseTreeIdent;

    token as_token -> AS;
    node identifier -> Identifier;
}

ast_node! {
    /// ```ebnf
    /// UseTreeNested =
    /// | "{" _ UseTree _ "}"
    /// | "{" _ ( UseTree _ "," )+ _ "}"
    /// | "{" _ ( UseTree _ "," )+ _ UseTree _ "}"
    /// ```
    struct UseTreeNested;

    token left_brace -> LEFT_BRACE;
    nodes trees -> UseTree;
    token right_brace -> RIGHT_BRACE;
}
