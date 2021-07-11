use danube_lex::LexIter;
use danube_token::{Span, Token, TokenKind};

macro_rules! float_literal {
    ($($expr:expr => $char:expr,)+) => {
        $(
            assert_eq!(
                Some(Ok(Token {
                    span: Span::new(0, $expr.len()),
                    kind: TokenKind::FloatLiteral($char)
                })),
                LexIter::new($expr).next(),
            );
        )+
    };
}

#[test]
fn simple() {
    float_literal! {
        ".0" => 0.0,
        ".1" => 0.1,
        ".12" => 0.12,
        "1." => 1.0,
        "1.0" => 1.0,
        "12.0" => 12.0,
        "12." => 12.0,
    };
}
