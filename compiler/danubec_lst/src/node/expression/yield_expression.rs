ast_node! {
    /// ```ebnf
    /// YieldExpression =
    /// | Expression _ "." _ "yield"
    /// ```
    struct YieldExpression;

    node expression -> Expression;
    token dot -> DOT;
    token yield_token -> YIELD;
}
