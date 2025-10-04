use crate::lex;

#[test]
fn punctuations() {
    let source = "\n\t~`!@#$%^&*-+=|:;,./?\\{}[]()<>";
    let tokens = lex(source);

    insta::assert_debug_snapshot!(tokens);
}

#[test]
fn r#char() {
    let source = "'a' '\\n' '\\'' '한' '\\u{AC00}'";
    let tokens = lex(source);

    insta::assert_debug_snapshot!(tokens);
}

#[test]
fn empty_string() {
    let source = r#""" """""""#;
    let tokens = lex(source);

    insta::assert_debug_snapshot!(tokens);
}

#[test]
fn non_multiline_string() {
    let source = r#""Hello, World!" "a${b}c""#;
    let tokens = lex(source);

    insta::assert_debug_snapshot!(tokens);
}

#[test]
fn multiline_string() {
    let source = r#""""Hello, "World"!""" """a${b}c""""#;
    let tokens = lex(source);

    insta::assert_debug_snapshot!(tokens);
}

#[test]
fn binary() {
    let source = "0b0101 0b_0101 0b__0101 0b01_01 0b01__01 0b0101_ 0b0101__";
    let tokens = lex(source);

    insta::assert_debug_snapshot!(tokens);
}

#[test]
fn octal() {
    let source =
        "0o01234567 0o_01234567 0o__01234567 0o01234_567 0o01234__567 0o01234567_ 0o01234567__";
    let tokens = lex(source);

    insta::assert_debug_snapshot!(tokens);
}

#[test]
fn hex() {
    let source = "0x0123456789ABCDEFabcdef 0x_0123456789ABCDEFabcdef 0x__0123456789ABCDEFabcdef 0x01234_56789ABCDEFabcdef 0x01234__56789ABCDEFabcdef 0x0123456789ABCDEFabcdef_ 0x0123456789ABCDEFabcdef__";
    let tokens = lex(source);

    insta::assert_debug_snapshot!(tokens);
}

#[test]
fn integer() {
    let source = "0 1 1234567890 0_1_2_3_4_5_6_7_8_9_0 0__1__2__3__4__5__6__7__8__9__0 1234567890_ 1234567890__";
    let tokens = lex(source);

    insta::assert_debug_snapshot!(tokens);
}

#[test]
fn float() {
    let source = "0.0 1.0 1234567890.0987654321 0_1_2_3_4_5_6_7_8_9_0.0_9_8_7_6_5_4_3_2_1 0__1__2__3__4__5__6__7__8__9__0.0__9__8__7__6__5__4__3__2__1 1234567890.0987654321_ 1234567890.0987654321__ 1.0e10 1.0E10 1.0e+10 1.0E+10 1.0e-10 1.0E-10 1.0e+10_9_87 1.0E+10__9__8__7 1.0e10_9_87 1.0E10__9__8__7 1e10 1E10 1e+10 1E+10 1e-10 1E-10 1e+10_9_87 1E+10__9__8__7 1e10_9_87 1E10__9__8__7";
    let tokens = lex(source);

    insta::assert_debug_snapshot!(tokens);
}

#[test]
fn identifier() {
    let source = "_ _foo _123 _foo123 foo foo_ foo123 foo123_ foo_foo foo_123 한글";
    let tokens = lex(source);

    insta::assert_debug_snapshot!(tokens);
}

#[test]
fn keywords() {
    let source = "as await break const continue crate else enum false fn for if impl in let loop match mut mod pub return Self self static struct super trait true type use where while yield";
    let tokens = lex(source);

    insta::assert_debug_snapshot!(tokens);
}
