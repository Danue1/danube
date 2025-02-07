crate::ast_node! {
    struct CharLiteral;

    tokens single_quotes -> SINGLE_QUOTE;
    node fragment -> CharLiteralFragment;
}

crate::ast_node! {
    enum CharLiteralFragment {
        Raw(Raw),
        EscapeSequence(CharLiteralEscapeSequence),
    }
}

crate::ast_node! {
    struct CharLiteralEscapeSequence;

    token backslash -> BACKSLASH;
    token raw -> Raw;
}
