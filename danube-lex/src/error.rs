use nom::error::{ErrorKind, ParseError};

#[derive(Debug, PartialEq)]
pub enum LexError<'a> {
    Nom(nom_locate::LocatedSpan<&'a str>, ErrorKind),
    Lex(&'a str),
}

impl<'a> ParseError<nom_locate::LocatedSpan<&'a str>> for LexError<'a> {
    fn from_error_kind(s: nom_locate::LocatedSpan<&'a str>, kind: ErrorKind) -> Self {
        LexError::Nom(s, kind)
    }

    fn append(_: nom_locate::LocatedSpan<&'a str>, _: ErrorKind, other: Self) -> Self {
        other
    }
}
