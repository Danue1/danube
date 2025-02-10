ast_node! {
    /// ```ebnf
    /// ImplDefinition =
    /// | "impl" TargetType "{" _ "}"
    /// | "impl" TargetType "{" _ ImplItemKind _ "}"
    /// | "impl" TargetType "{" ( _ ImplItemKind )+ _ "}"
    /// | "impl" TargetType "{" ( _ ImplItemKind )+ _ ImplItemKind _ "}"
    /// |
    /// | "impl" Type "for" TargetType "{" _ "}"
    /// | "impl" Type "for" TargetType "{" _ ImplItemKind _ "}"
    /// | "impl" Type "for" TargetType "{" ( _ ImplItemKind )+ _ "}"
    /// | "impl" Type "for" TargetType "{" ( _ ImplItemKind )+ _ ImplItemKind _ "}"
    /// ```
    struct ImplDefinition;

    token impl_token -> IMPL;
    node ty -> Type;
    token for_token -> FOR;
    node target_ty -> TargetType;
    token left_brace -> LEFT_BRACE;
    nodes items -> ImplItemKind;
    token right_brace -> RIGHT_BRACE;
}

ast_node! {
    /// ```ebnf
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

ast_node! {
    /// ```ebnf
    /// TargetType =
    /// | Type
    /// ```
    struct TargetType;

    node ty -> Type;
}
