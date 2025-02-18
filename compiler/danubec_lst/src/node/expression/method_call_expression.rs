ast_node! {
    /// ```ebnf
    /// MethodCallExpression =
    /// | Expression _ "." _ PathSegment _ "(" _ ")"
    /// | Expression _ "." _ PathSegment _ "(" _ Argument _ ")"
    /// | Expression _ "." _ PathSegment _ "(" ( _ Argument _ "," )+ _ ")"
    /// | Expression _ "." _ PathSegment _ "(" ( _ Argument _ "," )+ _ Argument _ ")"
    /// ```
    struct MethodCallExpression;

    node expression -> Expression;
    token dot -> DOT;
    node path_segment -> PathSegment;
    token left_paren -> LEFT_PAREN;
    nodes arguments -> Argument;
    token right_paren -> RIGHT_PAREN;
}
