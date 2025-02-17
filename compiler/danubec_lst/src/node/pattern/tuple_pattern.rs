ast_node! {
    /// ```ebnf
    /// TuplePattern =
    /// | "(" _ ")"
    /// | "(" _ TuplePatternElement _ ")"
    /// | "(" ( _ TuplePatternElement _ "," )+ _ ")"
    /// | "(" ( _ TuplePatternElement _ "," )+ TuplePatternElement _ ")"
    /// ```
    struct TuplePattern;

    token left_paren -> LEFT_PAREN;
    nodes elements -> TuplePatternElement;
    token right_paren -> RIGHT_PAREN;
}

ast_node! {
    /// ```ebnf
    /// TuplePatternElement =
    /// | Pattern
    /// ```
    struct TuplePatternElement;

    node pattern -> Pattern;
    token comma -> COMMA;
}
