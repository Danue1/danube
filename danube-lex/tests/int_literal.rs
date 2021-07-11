use danube_lex::LexIter;
use danube_token::{Span, Token, TokenKind};

macro_rules! int_literal {
    ($($expr:expr => $char:expr,)+) => {
        $(
            assert_eq!(
                Some(Ok(Token {
                    span: Span::new(0, $expr.len()),
                    kind: TokenKind::IntLiteral($char)
                })),
                LexIter::new($expr).next(),
            );
        )+
    };
}

#[test]
fn simple() {
    int_literal! {
        "1" => 1,
        "12" => 12,
        "123" => 123,
    };
}
