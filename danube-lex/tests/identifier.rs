use danube_lex::lex;
use danube_token::{Span, Token, TokenKind};

macro_rules! identifier {
    ($($expr:expr,)+) => {
        $(
            assert_eq!(
                Ok(vec![Token {
                    span: Span::new(0, $expr.len()),
                    kind: TokenKind::Identifier($expr.to_owned())
                }]),
                lex($expr)
            );
        )+
    };
}

#[test]
fn simple() {
    identifier!["a", "ab", "abc", "_a", "a_", "a1", "a1_", "a_1",];
}
