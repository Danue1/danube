ast_node! {
    /// ```ebnf
    /// Expression =
    /// | Expression _ "." _ "await"
    /// ```
    struct AwaitExpression;

    node expression -> Expression;
    token dot -> DOT;
    token await_token -> AWAIT;
}
