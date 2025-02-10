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

    node identifier -> Identifier;
    token as_token -> AS;
    node alias -> Identifier;
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
