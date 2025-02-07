crate::ast_node! {
    struct LetExpression;

    token let_token -> LET;
    node lhs -> Name;
    token equal -> EQUAL;
    node rhs -> Expression;
}
