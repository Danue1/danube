ast_node! {
    /// ```
    /// StringLiteral =
    /// | "\"" StringLiteralFragment* "\""
    /// ```
    struct StringLiteral;

    tokens double_quotes -> DOUBLE_QUOTE;
    nodes fragments -> StringLiteralFragment;
}

ast_node! {
    /// ```
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
    /// ```
    /// Escape =
    /// | "\\" Raw
    /// ```
    struct Escape;

    token backslash -> BACKSLASH;
    node raw -> Raw;
}

ast_node! {
    /// ```
    /// Interpolation =
    /// | "{" _ Expression _ "}"
    /// ```
    struct Interpolation;

    token left_brace -> LEFT_BRACE;
    node expression -> Expression;
    token right_brace -> RIGHT_BRACE;
}
