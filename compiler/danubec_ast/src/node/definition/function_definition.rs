ast_node! {
    /// ```ebnf
    /// FunctionDefinition =
    /// | "fn" _ Identifier _ FunctionParameters _ ";"
    /// | "fn" _ Identifier _ FunctionParameters _ BlockExpression
    ///
    /// | "fn" _ Identifier _ FunctionParameters _ FunctionReturnType _ ";"
    /// | "fn" _ Identifier _ FunctionParameters _ FunctionReturnType _ BlockExpression
    ///
    /// | "fn" _ Identifier _ TypeParameters _ FunctionParameters _ ";"
    /// | "fn" _ Identifier _ TypeParameters _ FunctionParameters _ BlockExpression
    ///
    /// | "fn" _ Identifier _ TypeParameters _ FunctionParameters _ FunctionReturnType _ ";"
    /// | "fn" _ Identifier _ TypeParameters _ FunctionParameters _ FunctionReturnType _ BlockExpression
    ///
    /// | "fn" _ Identifier _ TypeParameters _ FunctionParameters _ FunctionReturnType _ WhereClause _ ";"
    /// | "fn" _ Identifier _ TypeParameters _ FunctionParameters _ FunctionReturnType _ WhereClause _ BlockExpression
    ///
    /// FunctionReturnType =
    /// | "->" _ Type
    /// ```
    struct FunctionDefinition;

    token fn_token -> FN;
    node name -> Identifier;
    token left_chevron -> LEFT_CHEVRON;
    nodes type_parameters -> TypeParameter;
    token right_chevron -> RIGHT_CHEVRON;
    token left_paren -> LEFT_PAREN;
    nodes parameters -> FunctionParameter;
    token right_paren -> RIGHT_PAREN;
    token hyphen__right_chevron -> HYPHEN__RIGHT_CHEVRON;
    node return_type -> Type;
    token semicolon -> SEMICOLON;
    node body -> BlockExpression;
}

ast_node! {
    /// ```ebnf
    /// FunctionParameters =
    /// | "(" _ ")"
    /// | "(" _ FunctionParameter _ ")"
    /// | "(" _ ( FunctionParameter _ "," )+ _ ")"
    /// | "(" _ ( FunctionParameter _ "," )+ FunctionParameter _ ")"
    ///
    /// FunctionParameter =
    /// | Identifier _ ":" _ Type
    /// ```
    struct FunctionParameter;

    node name -> Identifier;
    token colon -> COLON;
    node ty -> Type;
    token comma -> COMMA;
}
