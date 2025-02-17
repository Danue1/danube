ast_node! {
    /// ```ebnf
    /// TraitDefinition =
    /// | "trait" _ Identifier _ "{" _ "}"
    /// | "trait" _ Identifier _ "{" _ ImplItemKinds _ "}"
    ///
    /// | "trait" Identifier _ TypeParameters _ "{" _ "}"
    /// | "trait" Identifier _ TypeParameters _ "{" _ ImplItemKinds _ "}"
    ///
    /// | "trait" _ Identifier _ WhereClause _ "{" _ "}"
    /// | "trait" _ Identifier _ WhereClause _ "{" _ ImplItemKinds _ "}"
    ///
    /// | "trait" Identifier _ TypeParameters _ WhereClause _ "{" _ "}"
    /// | "trait" Identifier _ TypeParameters _ WhereClause _ "{" _ ImplItemKinds _ "}"
    /// ```
    struct TraitDefinition;

    token trait_token -> TRAIT;
    node identifier -> Identifier;
    token left_chevron -> LEFT_CHEVRON;
    nodes type_parameters -> TypeParameter;
    token right_chevron -> RIGHT_CHEVRON;
    node where_clause -> WhereClause;
    token left_brace -> LEFT_BRACE;
    nodes items -> AssociatedItem;
    token right_brace -> RIGHT_BRACE;
}
