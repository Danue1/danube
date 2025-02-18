ast_node! {
    /// ```ebnf
    /// StructExpression =
    /// | Path _ "{" _ "}"
    /// | Path _ "{" _ StructExpressionField _ "}"
    /// | Path _ "{" ( _ StructExpressionField _ "," )+ _ "}"
    /// | Path _ "{" ( _ StructExpressionField _ "," )+ _ StructExpressionField _ "}"
    /// ```
    struct StructExpression;

    node path -> Path;
    token left_brace -> LEFT_BRACE;
    nodes fields -> StructExpressionField;
    token right_brace -> RIGHT_BRACE;
}

ast_node! {
    /// ```ebnf
    /// StructExpressionField =
    /// | Identifier _ ":" _ Expression
    /// ```
    struct StructExpressionField;

    node identifier -> Identifier;
    token colon -> COLON;
    node expression -> Expression;
}
