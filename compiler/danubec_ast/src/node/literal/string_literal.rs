ast_node! {
    /// ```ebnf
    /// StringLiteral =
    /// | "\"" StringLiteralFragment* "\""
    /// ```
    struct StringLiteral;

    tokens double_quotes -> DOUBLE_QUOTE;
    nodes fragments -> StringLiteralFragment;
}

ast_node! {
    /// ```ebnf
    /// StringLiteralFragment =
    /// | Raw
    /// | Escape
    /// | Interpolation
    /// ```
    enum StringLiteralFragment {
        Raw(Raw),
        Escape(Escape),
        Interpolation(Interpolation),
    }
}

ast_node! {
    /// ```ebnf
    /// Escape =
    /// | "\\" Raw
    /// ```
    struct Escape;

    token backslash -> BACKSLASH;
    node raw -> Raw;
}

ast_node! {
    /// ```ebnf
    /// Interpolation =
    /// | "{" _ Expression _ "}"
    /// ```
    struct Interpolation;

    token left_brace -> LEFT_BRACE;
    node expression -> Expression;
    token right_brace -> RIGHT_BRACE;
}
