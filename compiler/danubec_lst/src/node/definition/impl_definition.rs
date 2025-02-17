ast_node! {
    /// ```ebnf
    /// ImplDefinition =
    /// | "impl" Type "{" _ "}"
    /// | "impl" Type "{" _ AssociatedItems _ "}"
    /// | "impl" Type TargetType "{" _ "}"
    /// | "impl" Type TargetType "{" _ AssociatedItems _ "}"
    ///
    /// | "impl" TypeParameters Type "{" _ "}"
    /// | "impl" TypeParameters Type "{" _ AssociatedItems _ "}"
    /// | "impl" TypeParameters Type TargetType "{" _ "}"
    /// | "impl" TypeParameters Type TargetType "{" _ AssociatedItems _ "}"
    ///
    /// | "impl" TypeParameters Type WhereClause "{" _ "}"
    /// | "impl" TypeParameters Type WhereClause "{" _ AssociatedItems _ "}"
    /// | "impl" TypeParameters Type TargetType WhereClause "{" _ "}"
    /// | "impl" TypeParameters Type TargetType WhereClause "{" _ AssociatedItems _ "}"
    /// ```
    struct ImplDefinition;

    token impl_token -> IMPL;
    token left_chevron -> LEFT_CHEVRON;
    nodes type_parameters -> TypeParameter;
    token right_chevron -> RIGHT_CHEVRON;
    node ty -> Type;
    node target_type -> TargetType;
    node where_clause -> WhereClause;
    token left_brace -> LEFT_BRACE;
    nodes items -> AssociatedItem;
    token right_brace -> RIGHT_BRACE;
}

ast_node! {
    /// ```ebnf
    /// TargetType =
    /// | "for" Type
    struct TargetType;

    token for_token -> FOR;
    node ty -> Type;
}

ast_node! {
    /// ```ebnf
    /// AssociatedItems =
    /// | AssociatedItem
    /// | ( AssociatedItem _ )+ AssociatedItem
    ///
    /// AssociatedItem =
    /// | AssociatedItemKind
    /// | Visibility _ AssociatedItemKind
    /// ```
    struct AssociatedItem;

    node visibility -> Visibility;
    node kind -> AssociatedItemKind;
}

ast_node! {
    /// ```ebnf
    /// AssociatedItemKind =
    /// | FunctionDefinition
    /// | TypeDefinition
    /// | ConstDefinition
    /// ```
    enum AssociatedItemKind {
        Function(FunctionDefinition),
        Type(TypeDefinition),
        Const(ConstDefinition),
    }
}
