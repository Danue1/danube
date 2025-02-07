crate::ast_node! {
    struct NumericLiteral;

    node integer -> IntegerPart;
    node fraction -> FractionPart;
    node exponent -> Exponent;
}

crate::ast_node! {
    struct IntegerPart;

    node sign -> NumberSign;
    node encoding -> NumberEncoding;
    node fragments -> NumberFragment;
}

crate::ast_node! {
    struct FractionPart;

    token dot -> DOT;
    node fragments -> NumberFragment;
    node exponent -> Exponent;
}

crate::ast_node! {
    struct Exponent;

    token e -> E;
    node sign -> NumberSign;
    node fragments -> NumberFragment;
}

crate::ast_node! {
    enum NumberEncoding {
        Binary(Binary),
        Octal(Octal),
        Hexadecimal(Hexadecimal),
    }

    node binary -> Binary;
    node octal -> Octal;
    node hexadecimal -> Hexadecimal;
}

crate::ast_node! {
    struct Binary;

    token zero -> Raw;
}

crate::ast_node! {
    struct Octal;

    token zero -> Raw;
}

crate::ast_node! {
    struct Hexadecimal;

    node raw -> Raw;
}

crate::ast_node! {
    struct NumberSign;

    token minus -> HYPHEN;
    token plus -> PLUS;
}

crate::ast_node! {
    struct NumberFragment;

    token digit -> Raw;
    token underscore -> UNDERSCORE;
}
