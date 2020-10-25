use danube_lex::Tokens;
use nom::error::ErrorKind;

#[derive(Debug, PartialEq)]
pub enum ParseError {
    Nom(usize, usize, ErrorKind),
    Needed(nom::Needed),
    Parse(usize, usize, ErrorKind),
}

impl<'a> nom::error::ParseError<nom_locate::LocatedSpan<&'a str>> for ParseError {
    fn from_error_kind(s: nom_locate::LocatedSpan<&'a str>, kind: ErrorKind) -> Self {
        ParseError::Nom(s.location_offset(), s.location_line() as usize, kind)
    }

    fn append(_: nom_locate::LocatedSpan<&'a str>, _: ErrorKind, other: Self) -> Self {
        other
    }
}

impl<'a> nom::error::ParseError<Tokens<'a>> for ParseError {
    fn from_error_kind(s: Tokens, kind: ErrorKind) -> Self {
        ParseError::Parse(s.start, s.end, kind)
    }

    fn append(_: Tokens, _: ErrorKind, other: Self) -> Self {
        other
    }
}

impl<'a> From<nom::Err<ParseError>> for ParseError {
    fn from(e: nom::Err<ParseError>) -> ParseError {
        match e {
            nom::Err::Incomplete(needed) => ParseError::Needed(needed),
            nom::Err::Error(e) => e,
            nom::Err::Failure(e) => e,
        }
    }
}
