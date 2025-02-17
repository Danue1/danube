ast_node! {
    /// ```ebnf
    /// ImplDefinition =
    /// | "impl" Type "{" _ "}"
    /// | "impl" Type "{" _ ImplItemKinds _ "}"
    /// | "impl" Type "for" Type "{" _ "}"
    /// | "impl" Type "for" Type "{" _ ImplItemKinds _ "}"
    ///
    /// | "impl" TypeParameters Type "{" _ "}"
    /// | "impl" TypeParameters Type "{" _ ImplItemKinds _ "}"
    /// | "impl" TypeParameters Type WhereClause "{" _ "}"
    /// | "impl" TypeParameters Type WhereClause "{" _ ImplItemKinds _ "}"
    ///
    /// | "impl" TypeParameters Type "for" Type "{" _ "}"
    /// | "impl" TypeParameters Type "for" Type "{" _ ImplItemKinds _ "}"
    /// | "impl" TypeParameters Type "for" Type WhereClause "{" _ "}"
    /// | "impl" TypeParameters Type "for" Type WhereClause "{" _ ImplItemKinds _ "}"
    /// ```
    struct ImplDefinition;

    token impl_token -> IMPL;
    nodes type_parameters -> TypeParameter;
    // if types.len() == 1, then it is a target type.
    // if types.len() == 2, then it is a (trait type, target type).
    nodes types -> Type;
    token for_token -> FOR;
    node where_clause -> WhereClause;
    token left_brace -> LEFT_BRACE;
    nodes items -> ImplItemKind;
    token right_brace -> RIGHT_BRACE;
}

ast_node! {
    /// ```ebnf
    /// ImplItemKinds =
    /// | ImplItemKind
    /// | ( ImplItemKind _ )+ ImplItemKind
    ///
    /// ImplItemKind =
    /// | FunctionDefinition
    /// | TypeDefinition
    /// | ConstDefinition
    /// | StaticDefinition
    /// ```
    enum ImplItemKind {
        Function(FunctionDefinition),
        Type(TypeDefinition),
        Const(ConstDefinition),
        Static(StaticDefinition),
    }
}
