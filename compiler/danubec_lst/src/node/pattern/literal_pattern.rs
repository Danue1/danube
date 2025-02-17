ast_node! {
    /// ```ebnf
    /// LiteralPattern =
    /// | Literal
    /// ```
    struct LiteralPattern;

    node literal -> Literal;
}
