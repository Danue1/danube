use danube_lex::LexIter;
use danube_token::{Keyword, Span, Token, TokenKind};

macro_rules! keyword {
    ($($expr:expr => $keyword:ident,)+) => {
        $(
            assert_eq!(
                Some(Ok(Token {
                    span: Span::new(0, $expr.len()),
                    kind: TokenKind::Keyword(Keyword::$keyword)
                })),
                LexIter::new($expr).next(),
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
        "let" => Let,
        "mut" => Mutable,
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
        "package" => Package,
        "_" => Placeholder,
    };
}
