ast_node! {
    /// ```ebnf
    /// ArrayLiteral =
    /// | "[" _ ArrayLiteralElement* _ "]"
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
    /// | Expression _ ","
    /// ```
    struct ArrayLiteralElement;

    node value -> Expression;
    token comma -> COMMA;
}
