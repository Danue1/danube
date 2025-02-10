ast_node! {
    /// ```ebnf
    /// EnumDefinition =
    /// | "enum" _ Identifier _ "{" _ "}"
    /// | "enum" _ Identifier _ "{" _ EnumVariants _ "}"
    ///
    /// EnumVariants =
    /// | EnumVariant
    /// | ( EnumVariant _ "," )+
    /// | ( EnumVariant _ "," )+ EnumVariant
    /// ```
    struct EnumDefinition;

    token enum_token -> ENUM;
}

ast_node! {
    /// ```ebnf
    /// EnumVariant =
    /// | Identifier
    /// | Identifier _ EnumVariantKind
    /// ```
    struct EnumVariant;

    node identifier -> Identifier;
    node kind -> EnumVariantKind;
}

ast_node! {
    /// ```ebnf
    /// EnumVariantKind =
    /// | EnumVariantNamed
    /// | EnumVariantUnnamed
    /// | EnumVariantSequence
    /// ```
    enum EnumVariantKind {
        Named(EnumVariantNamed),
        Unnamed(EnumVariantUnnamed),
        Sequence(EnumVariantSequence),
    }
}

ast_node! {
    /// ```ebnf
    /// EnumVariantNamed =
    /// "{" _ "}"
    /// "{" _ EnumVariantNamedField _ "}"
    /// "{" _ ( EnumVariantNamedField _ "," )+ _ "}"
    /// "{" _ ( EnumVariantNamedField _ "," )+ _ EnumVariantNamedField _ "}"
    /// ```
    struct EnumVariantNamed;

    token left_brace -> LEFT_BRACE;
    nodes fields -> EnumVariantNamedField;
    token right_brace -> RIGHT_BRACE;
}

ast_node! {
    /// ```ebnf
    /// EnumVariantNamedField =
    /// | Identifier _ ":" _ Type
    /// ```
    struct EnumVariantNamedField;

    node identifier -> Identifier;
    token colon -> COLON;
    node ty -> Type;
    token comma -> COMMA;
}

ast_node! {
    /// ```ebnf
    /// EnumVariantUnnamed =
    /// "(" _ ")"
    /// "(" _ EnumVariantUnnamedField _ ")"
    /// "(" _ ( EnumVariantUnnamedField _ "," )+ _ ")"
    /// "(" _ ( EnumVariantUnnamedField _ "," )+ _ EnumVariantUnnamedField _ ")"
    /// ```
    struct EnumVariantUnnamed;

    token left_paren -> LEFT_PAREN;
    nodes fields -> EnumVariantUnnamedField;
    token right_paren -> RIGHT_PAREN;
}

ast_node! {
    /// ```ebnf
    /// EnumVariantUnnamedField =
    /// | Type
    /// ```
    struct EnumVariantUnnamedField;

    node ty -> Type;
    token comma -> COMMA;
}

ast_node! {
    /// ```ebnf
    /// EnumVariantSequence =
    /// "=" _ Expression
    /// ```
    struct EnumVariantSequence;

    token equal -> EQUAL;
    node expression -> Expression;
}
