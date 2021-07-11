use danube_lex::LexIter;
use danube_token::{Span, Token, TokenKind};

macro_rules! char_literal {
    ($($expr:expr => ($expected:expr, $count:expr),)+) => {
        $(
            assert_eq!(
                Some(Ok(Token {
                    span: Span::new(0, $count + 1),
                    kind: TokenKind::CharLiteral($expected)
                })),
                LexIter::new($expr).next(),
            );
        )+
    };
}

#[test]
fn simple() {
    char_literal! {
        "'a'" => ('a', 2),
        "'\\r'" => ('\r', 3),
        "'\\n'" => ('\n', 3),
        "'\\t'" => ('\t', 3),
    };
}
