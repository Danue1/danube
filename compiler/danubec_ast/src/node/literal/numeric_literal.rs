ast_node! {
    /// ```
    /// NumericLiteral =
    /// | NumericLiteralKind
    /// ```
    struct NumericLiteral;

    node kind -> NumericLiteralKind;
}

ast_node! {
    /// ```
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
    /// ```
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
    /// ```
    /// BinaryNumericLiteral =
    /// | "0b" NumericFragment
    /// ```
    struct BinaryNumericLiteral;

    token prefix -> NUMERIC_LITERAL_PREFIX;
    node fragment -> NumericFragment;
}

ast_node! {
    /// ```
    /// OctalNumericLiteral =
    /// | "0o" NumericFragment
    /// ```
    struct OctalNumericLiteral;

    token prefix -> NUMERIC_LITERAL_PREFIX;
    node fragment -> NumericFragment;
}

ast_node! {
    /// ```
    /// HexNumericLiteral =
    /// | "0x" NumericFragment
    /// ```
    struct HexNumericLiteral;

    token prefix -> NUMERIC_LITERAL_PREFIX;
    node fragment -> NumericFragment;
}

ast_node! {
    /// ```
    /// IntegerPart =
    /// | NumericFragment
    /// ```
    struct IntegerPart;

    node fragment -> NumericFragment;
}

ast_node! {
    /// ```
    /// FractionPart =
    /// | "." NumericFragment
    /// ```
    struct FractionPart;

    token dot -> DOT;
    node fragment -> NumericFragment;
}

ast_node! {
    /// ```
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
    /// ```
    /// NumericFragment =
    /// | [0-9]+
    /// | [0-9]+ ("_" [0-9]+)+
    struct NumericFragment;

    tokens digits -> NUMERIC;
    tokens underscores -> UNDERSCORE;
}
