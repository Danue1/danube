ast_node! {
    /// ```ebnf
    /// BooleanLiteral =
    /// | "true"
    /// | "false"
    /// ```
    struct BooleanLiteral;

    token true_token -> TRUE;
    token false_token -> FALSE;
}
