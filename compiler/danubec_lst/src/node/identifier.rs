ast_node! {
    /// ```ebnf
    /// Identifier =
    /// | [a-zA-Z_]
    /// | [a-zA-Z_] [a-zA-Z0-9_]+
    /// ```
    struct Identifier;
}
