ast_node! {
    /// ```ebnf
    /// ModuleDefinition =
    /// | "module" _ Identifier _ ";"
    ///
    /// | "module" _ Identifier _ "{" _ "}"
    /// | "module" _ Identifier _ "{" _ Definition _ "}"
    /// | "module" _ Identifier _ "{" ( _ Definition )+ _ "}"
    /// | "module" _ Identifier _ "{" ( _ Definition )+ _ Definition _ "}"
    /// ```
    struct ModuleDefinition;

    token mod_token -> MOD;
    node identifier -> Identifier;
    token left_brace -> LEFT_BRACE;
    nodes definitions -> Definition;
    token right_brace -> RIGHT_BRACE;
    token semicolon -> SEMICOLON;
}
