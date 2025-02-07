crate::ast_node! {
    struct ArrayLiteral;

    token left_bracket -> LEFT_BRACKET;
    nodes elements -> Expression;
    token right_bracket -> RIGHT_BRACKET;
}
