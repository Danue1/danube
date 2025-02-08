ast_node! {
    struct StringLiteral;

    tokens double_quotes -> DOUBLE_QUOTE;
    nodes fragments -> StringLiteralFragment;
}

ast_node! {
    enum StringLiteralFragment {
        Raw(Raw),
        Escape(Escape),
        Interpolation(Interpolation),
    }
}

ast_node! {
    struct Escape;

    token backslash -> BACKSLASH;
    node raw -> Raw;
}

ast_node! {
    struct Interpolation;

    token left_brace -> LEFT_BRACE;
    node expression -> Expression;
    token right_brace -> RIGHT_BRACE;
}
