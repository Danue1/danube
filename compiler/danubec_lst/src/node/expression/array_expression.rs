ast_node! {
    /// ```ebnf
    /// ArrayExpression =
    /// | "[" _ "]"
    /// | "[" _ ArrayExpressionElement _ "]"
    /// | "[" _ ArrayExpressionElement _ "," _ "]"
    /// | "[" ( _ ArrayExpressionElement _ "," )+ _ "]"
    /// | "[" ( _ ArrayExpressionElement _ "," )+ _ ArrayExpressionElement _ "]"
    /// ```
    struct ArrayExpression;

    token left_bracket -> LEFT_BRACKET;
    nodes elements -> ArrayElement;
    token right_bracket -> RIGHT_BRACKET;
}

ast_node! {
    /// ```ebnf
    /// ArrayElement =
    /// | Expression
    /// ```
    struct ArrayElement;

    node expression -> Expression;
    token comma -> COMMA;
}
