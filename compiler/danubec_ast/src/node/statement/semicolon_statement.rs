ast_node! {
    /// ```
    /// SemicolonStatement =
    /// | ";"
    /// ```
    struct SemicolonStatement;

    token semicolon -> SEMICOLON;
}
