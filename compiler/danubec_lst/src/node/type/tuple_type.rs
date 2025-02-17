ast_node! {
    /// ```ebnf
    /// TupleType =
    /// | "(" ")"
    /// | "(" TupleTypeElement ")"
    /// | "(" TupleTypeElement "," ")"
    /// | "(" ( TupleTypeElement "," )+ TupleTypeElement ")"
    struct TupleType;

    token left_bracket -> LEFT_BRACKET;
    nodes elements -> TupleTypeElement;
    token right_bracket -> RIGHT_BRACKET;
}

ast_node! {
    /// ```ebnf
    /// TupleTypeElement =
    /// | Type
    struct TupleTypeElement;

    node ty -> Type;
    token comma -> COMMA;
}
