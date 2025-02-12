ast_node! {
    /// ```ebnf
    /// UseDefinition =
    /// | "use" UseTreeKind _ ";"
    /// ```
    struct UseDefinition;

    token use_token -> USE;
    node kind -> UseTreeKind;
    token semicolon -> SEMICOLON;
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
    /// | Identifier
    /// | UseTreeIdent "as" _ Identifier
    /// ```
    struct UseTreeIdent;

    node lhs -> Identifier;
    token as_token -> AS;
    node rhs -> Identifier;
}

ast_node! {
    /// ```ebnf
    /// UseTreeNested =
    /// | "{" _ UseTreeKind _ "}"
    /// | "{" _ ( UseTreeKind _ "," )+ _ "}"
    /// | "{" _ ( UseTreeKind _ "," )+ _ UseTreeKind _ "}"
    /// ```
    struct UseTreeNested;

    token left_brace -> LEFT_BRACE;
    nodes kinds -> UseTreeKind;
    token right_brace -> RIGHT_BRACE;
}
