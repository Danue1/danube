ast_node! {
    /// ```ebnf
    /// NamedPattern =
    /// | Path _ "{" _ "}"
    /// | Path _ "{" _ NamedPatternElement _ "}"
    /// | Path _ "{" ( _ NamedPatternElement _ "," )+ _ "}"
    /// | Path _ "{" ( _ NamedPatternElement _ "," )+ _ NamedPatternElement _ "}"
    /// ```
    struct NamedPattern;

    node path -> Path;
    token left_brace -> LEFT_BRACE;
    nodes elements -> NamedPatternElement;
    token right_brace -> RIGHT_BRACE;
}

ast_node! {
    /// ```ebnf
    /// NamedPatternElement =
    /// | Path
    /// | Path _ ":" _ Pattern
    /// ```
    struct NamedPatternElement;

    node path -> Path;
    token colon -> COLON;
    node pattern -> Pattern;
    token comma -> COMMA;
}
