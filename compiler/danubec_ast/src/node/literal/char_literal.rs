ast_node! {
    /// ```
    /// CharLiteral =
    /// | "'" Raw "'"
    /// | "'" "\\" Raw "'"
    /// ```
    struct CharLiteral;

    tokens single_quotes -> SINGLE_QUOTE;
    token backslash -> BACKSLASH;
    node raw -> Raw;
}
