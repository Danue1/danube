ast_node! {
    /// ```ebnf
    /// NamedPattern =
    /// | Path _ "(" _ ")"
    /// | Path _ "(" _ NamedPatternElement _ ")"
    /// | Path _ "(" _ ( NamedPatternElement _ "," )+ _ ")"
    /// | Path _ "(" _ ( NamedPatternElement _ "," )+ _ NamedPatternElement _ ")"
    /// ```
    struct NamedPattern;

    node path -> Path;
    token left_paren -> LEFT_PAREN;
    nodes elements -> NamedPatternElement;
    token right_paren -> RIGHT_PAREN;
}

ast_node! {
    /// ```ebnf
    /// NamedPatternElement =
    /// | Pattern _ ","
    /// ```
    struct NamedPatternElement;

    node pattern -> Pattern;
    token comma -> COMMA;
}
