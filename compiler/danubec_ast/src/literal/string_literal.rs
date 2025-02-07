crate::ast_node! {
    struct StringLiteral;

    tokens double_quotes -> DOUBLE_QUOTE;
    nodes fragments -> StringLiteralFragment;
}

crate::ast_node! {
    enum StringLiteralFragment {
        Raw(Raw),
        Escape(Escape),
        Interpolation(Interpolation),
    }
}

crate::ast_node! {
    struct Escape;

    token backslash -> BACKSLASH;
    node raw -> Raw;
}

crate::ast_node! {
    struct Interpolation;

    token left_brace -> LEFT_BRACE;
    node expression -> Expression;
    token right_brace -> RIGHT_BRACE;
}
