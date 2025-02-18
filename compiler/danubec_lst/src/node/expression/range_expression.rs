ast_node! {
    /// ```ebnf
    /// RangeExpression =
    /// | ".."
    /// | ".." _ Expression
    /// | "..=" _ Expression
    /// | Expression _ ".."
    /// | Expression _ ".." _ Expression
    /// | Expression _ "..=" _ Expression
    /// ```
    struct RangeExpression;

    node start -> RangeExpressionRhs;
    node range_operator -> RangeOperator;
    node end -> Expression;
}

ast_node! {
    /// ```ebnf
    /// RangeExpressionRhs =
    /// | Expression
    /// ```
    struct RangeExpressionRhs;

    node expression -> Expression;
}

ast_node! {
    /// ```ebnf
    /// RangeOperator =
    /// | ".."
    /// | "..="
    /// ```
    struct RangeOperator;

    token DOT__DOT -> DOT__DOT;
    token DOT__DOT__EQUAL -> DOT__DOT__EQUAL;
}
