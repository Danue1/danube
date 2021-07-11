use danube_lex::LexIter;
use danube_token::{Span, Token, TokenKind};

macro_rules! identifier {
    ($($expr:expr,)+) => {
        $(
            assert_eq!(
                Some(Ok(Token {
                    span: Span::new(0, $expr.len()),
                    kind: TokenKind::Identifier($expr.to_owned())
                })),
                LexIter::new($expr).next(),
            );
        )+
    };
}

#[test]
fn simple() {
    identifier!["a", "ab", "abc", "_a", "a_", "a1", "a1_", "a_1",];
}
