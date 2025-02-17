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
    nodes elements -> ArrayExpressionElement;
    token right_bracket -> RIGHT_BRACKET;
}

ast_node! {
    /// ```ebnf
    /// ArrayElement =
    /// | Expression
    /// ```
    struct ArrayExpressionElement;

    node expression -> Expression;
    token comma -> COMMA;
}
