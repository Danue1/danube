ast_node! {
    /// ```
    /// Root =
    /// | ( _ Definition )*
    /// ```
    struct Root;

    nodes definitions -> Definition;
}
