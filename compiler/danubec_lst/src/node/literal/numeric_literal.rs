ast_node! {
    /// ```ebnf
    /// NumericLiteral =
    /// | NumericLiteralKind
    /// ```
    struct NumericLiteral;

    node kind -> NumericLiteralKind;
}

ast_node! {
    /// ```ebnf
    /// NumericLiteralKind =
    /// | DecimalNumericLiteral
    /// | BinaryNumericLiteral
    /// | OctalNumericLiteral
    /// | HexNumericLiteral
    /// ```
    enum NumericLiteralKind {
        Decimal(DecimalNumericLiteral),
        Binary(BinaryNumericLiteral),
        Octal(OctalNumericLiteral),
        Hex(HexNumericLiteral),
    }
}

ast_node! {
    /// ```ebnf
    /// DecimalNumericLiteral =
    /// | IntegerPart
    /// | IntegerPart FractionPart
    /// | IntegerPart ExponentPart
    /// | IntegerPart FractionPart ExponentPart
    /// ```
    struct DecimalNumericLiteral;

    node integer -> IntegerPart;
    node fraction -> FractionPart;
    node exponent -> ExponentPart;
}

ast_node! {
    /// ```ebnf
    /// BinaryNumericLiteral =
    /// | "0b" NumericFragment
    /// ```
    struct BinaryNumericLiteral;

    token prefix -> NUMERIC_LITERAL_PREFIX;
    node fragment -> NumericFragment;
}

ast_node! {
    /// ```ebnf
    /// OctalNumericLiteral =
    /// | "0o" NumericFragment
    /// ```
    struct OctalNumericLiteral;

    token prefix -> NUMERIC_LITERAL_PREFIX;
    node fragment -> NumericFragment;
}

ast_node! {
    /// ```ebnf
    /// HexNumericLiteral =
    /// | "0x" NumericFragment
    /// ```
    struct HexNumericLiteral;

    token prefix -> NUMERIC_LITERAL_PREFIX;
    node fragment -> NumericFragment;
}

ast_node! {
    /// ```ebnf
    /// IntegerPart =
    /// | NumericFragment
    /// ```
    struct IntegerPart;

    node fragment -> NumericFragment;
}

ast_node! {
    /// ```ebnf
    /// FractionPart =
    /// | "." NumericFragment
    /// ```
    struct FractionPart;

    token dot -> DOT;
    node fragment -> NumericFragment;
}

ast_node! {
    /// ```ebnf
    /// ExponentPart =
    /// | "e" NumericFragment
    /// | "e" "-" NumericFragment
    /// | "e" "+" NumericFragment
    /// | "E" NumericFragment
    /// | "E" "-" NumericFragment
    /// | "E" "+" NumericFragment
    /// ```
    struct ExponentPart;

    token e -> E;
    token sign -> ExponentPartSign;
    node fragment -> NumericFragment;
}

ast_node! {
    /// ```ebnf
    /// NumericFragment =
    /// | [0-9]+
    /// | [0-9]+ ("_" [0-9]+)+
    struct NumericFragment;

    tokens digits -> NUMERIC;
    tokens underscores -> UNDERSCORE;
}
