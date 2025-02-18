ast_node! {
    /// ```ebnf
    /// IndexExpression =
    /// | Expression _ "[" _ "]"
    /// | Expression _ "[" _ IndexElement _ "]"
    /// | Expression _ "[" _ IndexElement _ "," _ "]"
    /// ```
    struct IndexExpression;

    node expression -> Expression;
    token left_bracket -> LEFT_BRACKET;
    node index -> IndexElement;
    token right_bracket -> RIGHT_BRACKET;
}

ast_node! {
    /// ```ebnf
    /// Argument =
    /// | Expression
    /// ```
    struct IndexElement;

    node expression -> Expression;
}
