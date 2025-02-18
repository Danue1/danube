ast_node! {
    /// ```ebnf
    /// Expression =
    /// | Expression _ "(" _ ")"
    /// | Expression _ "(" _ Expression _ ")"
    /// | Expression _ "(" ( _ Expression _ "," )+ _ ")"
    /// | Expression _ "(" ( _ Expression _ "," )+ _ Expression _ ")"
    /// ```
    struct CallExpression;

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
