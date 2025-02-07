crate::ast_node! {
    enum NumericLiteral {
        Decimal(DecimalNumericLiteral),
        Binary(BinaryNumericLiteral),
        Octal(OctalNumericLiteral),
        Hex(HexNumericLiteral),
    }
}

crate::ast_node! {
    struct DecimalNumericLiteral;

    node integer -> IntegerPart;
    node fraction -> FractionPart;
    node exponent -> Exponent;
}

crate::ast_node! {
    struct BinaryNumericLiteral;

    token prefix -> NUMERIC_LITERAL_PREFIX;
    node fragment -> NumericFragment;
}

crate::ast_node! {
    struct OctalNumericLiteral;

    token prefix -> NUMERIC_LITERAL_PREFIX;
    node fragment -> NumericFragment;
}

crate::ast_node! {
    struct HexNumericLiteral;

    token prefix -> NUMERIC_LITERAL_PREFIX;
    node fragment -> NumericFragment;
}

crate::ast_node! {
    struct IntegerPart;

    node fragment -> NumericFragment;
}

crate::ast_node! {
    struct FractionPart;

    token dot -> DOT;
    node fragment -> NumericFragment;
}

crate::ast_node! {
    struct Exponent;

    token e -> E;
    token minus -> HYPHEN;
    token plus -> PLUS;
    node fragment -> NumericFragment;
}

crate::ast_node! {
    struct NumericFragment;

    tokens digits -> NUMERIC;
    tokens underscores -> UNDERSCORE;
}
