ast_node! {
    /// ```ebnf
    /// PlaceholderPattern =
    /// | "_"
    /// ```
    struct PlaceholderPattern;

    token underscore -> UNDERSCORE;
}
