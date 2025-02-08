ast_node! {
    enum NumericLiteral {
        Decimal(DecimalNumericLiteral),
        Binary(BinaryNumericLiteral),
        Octal(OctalNumericLiteral),
        Hex(HexNumericLiteral),
    }
}

ast_node! {
    struct DecimalNumericLiteral;

    node integer -> IntegerPart;
    node fraction -> FractionPart;
    node exponent -> Exponent;
}

ast_node! {
    struct BinaryNumericLiteral;

    token prefix -> NUMERIC_LITERAL_PREFIX;
    node fragment -> NumericFragment;
}

ast_node! {
    struct OctalNumericLiteral;

    token prefix -> NUMERIC_LITERAL_PREFIX;
    node fragment -> NumericFragment;
}

ast_node! {
    struct HexNumericLiteral;

    token prefix -> NUMERIC_LITERAL_PREFIX;
    node fragment -> NumericFragment;
}

ast_node! {
    struct IntegerPart;

    node fragment -> NumericFragment;
}

ast_node! {
    struct FractionPart;

    token dot -> DOT;
    node fragment -> NumericFragment;
}

ast_node! {
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
