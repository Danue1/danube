ast_node! {
    /// ```
    /// BooleanLiteral =
    /// | "true"
    /// | "false"
    /// ```
    struct BooleanLiteral;

    token true_token -> TRUE;
    token false_token -> FALSE;
}
