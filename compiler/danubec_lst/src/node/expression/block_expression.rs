ast_node! {
    /// ```ebnf
    /// BlockExpression =
    /// | "{" _ "}"
    /// | "{" _ Statement _ "}"
    /// | "{" _ Statement ( _ Statement )+ _ "}"
    /// ```
    struct BlockExpression;

    token left_brace -> LEFT_BRACE;
    nodes statements -> Statement;
    token right_brace -> RIGHT_BRACE;
}
