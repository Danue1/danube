ast_node! {
    /// ```ebnf
    /// NeverPattern =
    /// | "!"
    /// ```
    struct NeverPattern;

    token exclamation -> EXCLAMATION;
}
