ast_node! {
    /// ```
    /// AssignmentExpression =
    /// | Expression _ "=" _ Expression
    /// ```
    struct AssignmentExpression;

    node lhs -> Expression;
    token equal -> EQUAL;
    node rhs -> Expression;
}
