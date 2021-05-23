use danube_lex::lex;
use danube_token::{Span, Token, TokenKind};

macro_rules! char_literal {
    ($($expr:expr => ($char:expr, $count:expr),)+) => {
        $(
            assert_eq!(
                Ok(vec![Token {
                    span: Span::new(0, $count + 2),
                    kind: TokenKind::CharLiteral($char)
                }]),
                lex($expr)
            );
        )+
    };
}

#[test]
fn simple() {
    char_literal! {
        "'a'" => ('a', 1),
        "'\\r'" => ('\r', 2),
        "'\\n'" => ('\n', 2),
        "'\\t'" => ('\t', 2),
    };
}
