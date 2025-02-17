ast_node! {
    /// ```ebnf
    /// ArrayLiteral =
    /// | "[" _ "]"
    /// | "[" _ ArrayLiteralElement _ "]"
    /// | "[" _ ArrayLiteralElement _ "," _ "]"
    /// | "[" ( _ ArrayLiteralElement _ "," )+ _ "]"
    /// | "[" ( _ ArrayLiteralElement _ "," )+ _ ArrayLiteralElement _ "]"
    /// ```
    struct ArrayLiteral;

    token left_bracket -> LEFT_BRACKET;
    nodes elements -> Expression;
    token right_bracket -> RIGHT_BRACKET;
}

ast_node! {
    /// ```ebnf
    /// ArrayLiteralElement =
    /// | Expression
    /// ```
    struct ArrayLiteralElement;

    node value -> Expression;
    token comma -> COMMA;
}
