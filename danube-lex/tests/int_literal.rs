use danube_lex::lex;
use danube_token::{Position, Token, TokenKind};

macro_rules! int_literal {
    ($($expr:expr => $char:expr,)+) => {
        $(
            assert_eq!(
                Ok(vec![Token {
                    position: Position::new(0, $expr.len()),
                    kind: TokenKind::IntLiteral($char)
                }]),
                lex($expr)
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
