crate::ast_node! {
    struct FunctionDefinition;

    token fn_token -> FN;
    node name -> Name;
    token left_paren -> LEFT_PAREN;
    nodes parameters -> FunctionParameter;
    token right_paren -> RIGHT_PAREN;
    token hyphen__right_chevron -> HYPHEN__RIGHT_CHEVRON;
    node return_type -> Type;
    token semicolon -> SEMICOLON;
    node body -> BlockExpression;
}

crate::ast_node! {
    struct FunctionParameter;

    node name -> Name;
    token colon -> COLON;
    node ty -> Type;
    token comma -> COMMA;
}
