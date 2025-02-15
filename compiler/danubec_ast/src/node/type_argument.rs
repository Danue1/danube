ast_node! {
    /// ```ebnf
    /// TypeArguments =
    /// | "<" _ Type _ ">"
    /// | "<" _ Type _ "," _ ">"
    /// | "<" ( _ Type _ "," )+ _ ">"
    /// | "<" ( _ Type _ "," )+ Type _ ">"
    /// ```
    struct TypeArgument;

    token left_chevron -> LEFT_CHEVRON;
    nodes types -> Type;
    token right_chevron -> RIGHT_CHEVRON;
}
