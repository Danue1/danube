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
    nodes types -> Type;
    token for_token -> FOR;
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
