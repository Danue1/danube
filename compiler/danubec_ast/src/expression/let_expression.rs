crate::ast_node! {
    struct LetExpression;

    token let_token -> LET;
    node lhs -> Identifier;
    token equal -> EQUAL;
    node rhs -> Expression;
}
