ast_node! {
    /// ```ebnf
    /// FieldExpression =
    /// | Expression _ "." _ Identifier
    /// | Expression _ "." _ [0-9]+
    /// ```
    struct FieldExpression;

    node expression -> Expression;
    token dot -> DOT;
    node identifier -> Identifier;
    token numeric -> NUMERIC;
}
