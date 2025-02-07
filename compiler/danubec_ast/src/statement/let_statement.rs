crate::ast_node! {
    struct LetStatement;

    token let_token -> LET;
    node lhs -> Name;
    token colon -> COLON;
    node ty -> Type;
    token equal -> EQUAL;
    node rhs -> Expression;
    token semicolon -> SEMICOLON;
}
