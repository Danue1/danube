ast_node! {
    /// ```ebnf
    /// ArrayPattern =
    /// | "[" _ "]"
    /// | "[" _ ArrayPatternElement _ "]"
    /// | "[" ( _ ArrayPatternElement _ "," )+ _ "]"
    /// | "[" ( _ ArrayPatternElement _ "," )+ ArrayPatternElement _ "]"
    /// ```
    struct ArrayPattern;

    token left_bracket -> LEFT_BRACKET;
    nodes elements -> ArrayPatternElement;
    token right_bracket -> RIGHT_BRACKET;
}

ast_node! {
    /// ```ebnf
    /// ArrayPatternElement =
    /// | Pattern
    /// ```
    struct ArrayPatternElement;

    node pattern -> Pattern;
    token comma -> COMMA;
}
