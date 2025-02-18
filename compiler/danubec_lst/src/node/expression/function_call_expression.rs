ast_node! {
    /// ```ebnf
    /// FunctionCallExpression =
    /// | Expression _ "(" _ ")"
    /// | Expression _ "(" _ Argument _ ")"
    /// | Expression _ "(" ( _ Argument _ "," )+ _ ")"
    /// | Expression _ "(" ( _ Argument _ "," )+ _ Argument _ ")"
    /// ```
    struct FunctionCallExpression;

    node expression -> Expression;
    token left_paren -> LEFT_PAREN;
    nodes arguments -> Argument;
    token right_paren -> RIGHT_PAREN;
}

ast_node! {
    /// ```ebnf
    /// Argument =
    /// | Expression
    /// ```
    struct Argument;

    node expression -> Expression;
    token comma -> COMMA;
}
