ast_node! {
    struct AssignmentExpression;

    node lhs -> Expression;
    token equal -> EQUAL;
    node rhs -> Expression;
}
