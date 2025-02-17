ast_node! {
    /// ```ebnf
    /// OrPattern =
    /// | ( Pattern _ "|" _ )+ Pattern
    /// ```
    struct OrPattern;

    nodes patterns -> Pattern;
    tokens pipe -> PIPE;
}
