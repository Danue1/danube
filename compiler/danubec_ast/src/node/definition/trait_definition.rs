ast_node! {
    /// ```ebnf
    /// TraitDefinition =
    /// | "trait" _ Identifier _ "{" _ "}"
    /// | "trait" _ Identifier _ "{" _ ImplItemKind _ "}"
    /// | "trait" _ Identifier _ "{" ( _ ImplItemKind )+ _ "}"
    /// | "trait" _ Identifier _ "{" ( _ ImplItemKind )+ _ ImplItemKind _ "}"
    /// ```
    struct TraitDefinition;

    token trait_token -> TRAIT;
    node identifier -> Identifier;
    token left_brace -> LEFT_BRACE;
    nodes items -> ImplItemKind;
    token right_brace -> RIGHT_BRACE;
}
