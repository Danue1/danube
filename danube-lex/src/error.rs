use nom::error::{ErrorKind, ParseError};

#[derive(Debug, PartialEq)]
pub enum LexError {
    Nom(usize, usize, ErrorKind),
    Lex(usize, usize),
}

impl<'a> ParseError<nom_locate::LocatedSpan<&'a str>> for LexError {
    fn from_error_kind(s: nom_locate::LocatedSpan<&'a str>, kind: ErrorKind) -> Self {
        LexError::Nom(s.location_offset(), s.location_line() as usize, kind)
    }

    fn append(_: nom_locate::LocatedSpan<&'a str>, _: ErrorKind, other: Self) -> Self {
        other
    }
}
