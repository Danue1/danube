ast_node! {
    /// ```ebnf
    /// SliceType =
    /// | "[" Type "]"
    struct SliceType;

    token left_bracket -> LEFT_BRACKET;
    node ty -> Type;
    token right_bracket -> RIGHT_BRACKET;
}
