ast_node! {
    /// ```ebnf
    /// RestPattern =
    /// | ".."
    /// ```
    struct RestPattern;

    token dot_dot -> DOT__DOT;
}
