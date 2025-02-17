ast_node! {
    /// ```ebnf
    /// TypeDefinition =
    /// | "type" _ Identifier _ "=" _ Type _ ";"
    /// | "type" _ Identifier _ TypeParameters _ "=" _ Type _ ";"
    /// | "type" _ Identifier _ WhereClause _ "=" _ Type _ ";"
    /// | "type" _ Identifier _ TypeParameters _ WhereClause _ "=" _ Type _ ";"
    /// ```
    struct TypeDefinition;

    token ty_token -> TYPE;
    node identifier -> Identifier;
    token left_chevron -> LEFT_CHEVRON;
    nodes type_parameters -> TypeParameter;
    token right_chevron -> RIGHT_CHEVRON;
    node where_clause -> WhereClause;
    token equal -> EQUAL;
    node ty -> Type;
    token semicolon -> SEMICOLON;
}

ast_node! {
    /// ```ebnf
    /// TypeParameters =
    /// | "<" _ ">"
    /// | "<" _ TypeParameter _ ">"
    /// | "<" _ ( TypeParameter _ "," )+ _ ">"
    /// | "<" _ ( TypeParameter _ "," )+ TypeParameter _ ">"
    ///
    /// TypeParameter =
    /// | Identifier
    /// | Identifier _ ":"
    /// | Identifier _ ":" _ Type
    /// | Identifier _ ":" ( _ Type _ "+" )+
    /// | Identifier _ ":" ( _ Type _ "+" )+ _ Type
    /// ```
    struct TypeParameter;

    node identifier -> Identifier;
    token colon -> COLON;
    nodes types -> Type;
    token comma -> COMMA;
}

ast_node! {
    /// WhereClause =
    /// | "where"
    /// | "where" _ TypeConstraint
    /// | "where" _ ( TypeConstraint _ "," )+ _ TypeConstraint
    struct WhereClause;

    token where_token -> WHERE;
    nodes type_constraints -> TypeConstraint;
}

ast_node! {
    /// ```ebnf
    /// TypeConstraint =
    /// | Type
    /// | Type _ ":" _ TypeConstraintParameter
    /// ```
    struct TypeConstraint;

    node lhs -> Type;
    token colon -> COLON;
    node rhs -> TypeConstraintParameter;
    token plus -> PLUS;
}

ast_node! {
    /// ```ebnf
    /// TypeConstraintParameter =
    /// | Type
    /// | ( Type _ "+" )+
    /// | ( Type _ "+" )+ _ Type
    /// ```
    struct TypeConstraintParameter;

    nodes types -> Type;
    token plus -> PLUS;
}
