use nom::error::{ErrorKind, ParseError};

#[derive(Debug, PartialEq)]
pub enum Error {
    Nom(usize, usize, ErrorKind),
    Needed(nom::Needed),
    Lex(usize, usize),
}

impl<'a> ParseError<nom_locate::LocatedSpan<&'a str>> for Error {
    fn from_error_kind(s: nom_locate::LocatedSpan<&'a str>, kind: ErrorKind) -> Self {
        Error::Nom(s.location_offset(), s.location_line() as usize, kind)
    }

    fn append(_: nom_locate::LocatedSpan<&'a str>, _: ErrorKind, other: Self) -> Self {
        other
    }
}

impl<'a> From<nom::Err<Error>> for Error {
    fn from(e: nom::Err<Error>) -> Error {
        match e {
            nom::Err::Incomplete(needed) => Error::Needed(needed),
            nom::Err::Error(e) => e,
            nom::Err::Failure(e) => e,
        }
    }
}
