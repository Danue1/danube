use danube_lex::Tokens;
use nom::error::ErrorKind;

#[derive(Debug, PartialEq)]
pub enum Error {
    Nom(usize, usize, ErrorKind),
    Needed(nom::Needed),
    Lex(danube_lex::Error),
    Parser(usize, usize, ErrorKind),
}

impl<'a> nom::error::ParseError<nom_locate::LocatedSpan<&'a str>> for Error {
    fn from_error_kind(s: nom_locate::LocatedSpan<&'a str>, kind: ErrorKind) -> Self {
        Error::Nom(s.location_offset(), s.location_line() as usize, kind)
    }

    fn append(_: nom_locate::LocatedSpan<&'a str>, _: ErrorKind, other: Self) -> Self {
        other
    }
}

impl<'a> nom::error::ParseError<Tokens<'a>> for Error {
    fn from_error_kind(s: Tokens, kind: ErrorKind) -> Self {
        Error::Parser(s.start, s.end, kind)
    }

    fn append(_: Tokens, _: ErrorKind, other: Self) -> Self {
        other
    }
}

impl From<danube_lex::Error> for Error {
    fn from(error: danube_lex::Error) -> Error {
        Error::Lex(error)
    }
}

impl<'a> From<nom::Err<Error>> for Error {
    fn from(error: nom::Err<Error>) -> Error {
        match error {
            nom::Err::Incomplete(needed) => Error::Needed(needed),
            nom::Err::Error(error) | nom::Err::Failure(error) => error,
        }
    }
}
