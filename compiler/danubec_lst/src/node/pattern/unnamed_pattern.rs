ast_node! {
    /// ```ebnf
    /// UnnamedPattern =
    /// | Path _ "(" _ ")"
    /// | Path _ "(" _ Pattern _ ")"
    /// | Path _ "(" ( _ Pattern _ "," )+ _ ")"
    /// | Path _ "(" ( _ Pattern _ "," )+ _ Pattern _ ")"
    /// ```
    struct UnnamedPattern;

    node path -> Path;
    token left_paren -> LEFT_PAREN;
    nodes elements -> UnnamedPatternElement;
    token right_paren -> RIGHT_PAREN;
}

ast_node! {
    /// ```ebnf
    /// UnnamedPatternElement =
    /// | Pattern
    /// ```
    struct UnnamedPatternElement;

    node pattern -> Pattern;
    token comma -> COMMA;
}
