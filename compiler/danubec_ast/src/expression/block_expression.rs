crate::ast_node! {
    struct BlockExpression;

    token left_brace -> LEFT_BRACE;
    nodes statements -> Statement;
    token right_brace -> RIGHT_BRACE;
}
