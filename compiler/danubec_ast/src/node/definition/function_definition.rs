ast_node! {
    /// ```
    /// FunctionDefinition =
    /// | "fn" _ Identifier _  "(" _ FunctionParameter* _ ")" _ ";"
    /// | "fn" _ Identifier _  "(" _ FunctionParameter* _ ")" _ "->" _ Type _ ";"
    /// | "fn" _ Identifier _  "(" _ FunctionParameter* _ ")" _ BlockExpression
    /// | "fn" _ Identifier _  "(" _ FunctionParameter* _ ")" _ "->" _ Type _ BlockExpression
    /// ```
    struct FunctionDefinition;

    token fn_token -> FN;
    node name -> Identifier;
    token left_paren -> LEFT_PAREN;
    nodes parameters -> FunctionParameter;
    token right_paren -> RIGHT_PAREN;
    token hyphen__right_chevron -> HYPHEN__RIGHT_CHEVRON;
    node return_type -> Type;
    token semicolon -> SEMICOLON;
    node body -> BlockExpression;
}

ast_node! {
    /// ```
    /// FunctionParameter =
    /// | Identifier _ ":" _ Type (_ ",")?
    /// ```
    struct FunctionParameter;

    node name -> Identifier;
    token colon -> COLON;
    node ty -> Type;
    token comma -> COMMA;
}
