ast_node! {
    /// ```ebnf
    /// SemicolonStatement =
    /// | ";"
    /// ```
    struct SemicolonStatement;

    token semicolon -> SEMICOLON;
}
