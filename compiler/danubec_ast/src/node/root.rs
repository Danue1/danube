ast_node! {
    /// ```ebnf
    /// Root =
    /// | ( _ Definition )*
    /// ```
    struct Root;

    nodes definitions -> Definition;
}
