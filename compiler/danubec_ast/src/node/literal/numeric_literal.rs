ast_node! {
    /// ```
    /// NumericLiteral =
    /// | DecimalNumericLiteral
    /// | BinaryNumericLiteral
    /// | OctalNumericLiteral
    /// | HexNumericLiteral
    /// ```
    enum NumericLiteral {
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
    /// | IntegerPart Exponent
    /// | IntegerPart FractionPart Exponent
    /// ```
    struct DecimalNumericLiteral;

    node integer -> IntegerPart;
    node fraction -> FractionPart;
    node exponent -> Exponent;
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
    /// Exponent =
    /// | "e" NumericFragment
    /// | "e" "-" NumericFragment
    /// | "e" "+" NumericFragment
    /// | "E" NumericFragment
    /// | "E" "-" NumericFragment
    /// | "E" "+" NumericFragment
    /// ```
    struct Exponent;

    token e -> E;
    token minus -> HYPHEN;
    token plus -> PLUS;
    node fragment -> NumericFragment;
}

ast_node! {
    struct NumericFragment;

    tokens digits -> NUMERIC;
    tokens underscores -> UNDERSCORE;
}
