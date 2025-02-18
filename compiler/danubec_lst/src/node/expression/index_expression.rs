ast_node! {
    /// ```ebnf
    /// IndexExpression =
    /// | Expression _ "[" _ "]"
    /// | Expression _ "[" _ Expression _ "]"
    /// | Expression _ "[" ( _ Expression _ "," )+ _ "]"
    /// | Expression _ "[" ( _ Expression _ "," )+ _ Expression _ "]"
    /// ```
    struct IndexExpression;

    node expression -> Expression;
    token left_bracket -> LEFT_BRACKET;
    nodes arguments -> Argument;
    token right_bracket -> RIGHT_BRACKET;
}
