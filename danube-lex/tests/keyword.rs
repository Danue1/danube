use danube_lex::lex;
use danube_token::{Keyword, Position, Token, TokenKind};

macro_rules! keyword {
    ($($expr:expr => $keyword:ident,)+) => {
        $(
            assert_eq!(
                Ok(vec![Token {
                    position: Position::new(0, $expr.len()),
                    kind: TokenKind::Keyword(Keyword::$keyword)
                }]),
                lex($expr)
            );
        )+
    };
}

#[test]
fn simple() {
    keyword! {
        "if" => If,
        "else" => Else,
        "for" => For,
        "while" => While,
        "loop" => Loop,
        "in" => In,
        "break" => Break,
        "continue" => Continue,
        "match" => Match,
        "return" => Return,
        "yield" => Yield,
        "where" => Where,
        "const" => Const,
        "static" => Static,
        "let" => Let,
        "mut" => Mut,
        "fn" => Function,
        "trait" => Trait,
        "type" => Type,
        "enum" => Enum,
        "impl" => Impl,
        "mod" => Module,
        "Self" => TypeSelf,
        "self" => VariableSelf,
        "pub" => Public,
        "await" => Await,
        "use" => Use,
        "super" => Super,
        "as" => As,
        "_" => Placeholder,
    };
}
