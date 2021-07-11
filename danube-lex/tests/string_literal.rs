use danube_lex::LexIter;
use danube_token::{Span, Token, TokenKind};

macro_rules! string_literal {
    ($($expr:expr,)+) => {
        $(
            assert_eq!(
                Some(Ok(Token {
                    span: Span::new(0, $expr.len()),
                    kind: TokenKind::StringLiteral($expr[1..$expr.len() - 1].to_owned())
                })),
                LexIter::new($expr).next(),
            );
        )+
    };
}

#[test]
fn simple() {
    string_literal![r#""Hello""#,];
}
