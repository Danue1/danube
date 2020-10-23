use danube_lex::Tokens;
use nom::error::ErrorKind;

#[derive(Debug, PartialEq)]
pub enum ParseError<'a> {
    Nom(nom_locate::LocatedSpan<&'a str>, ErrorKind),
    Parse(Tokens<'a>, ErrorKind),
}

impl<'a> nom::error::ParseError<nom_locate::LocatedSpan<&'a str>> for ParseError<'a> {
    fn from_error_kind(s: nom_locate::LocatedSpan<&'a str>, kind: ErrorKind) -> Self {
        ParseError::Nom(s, kind)
    }

    fn append(_: nom_locate::LocatedSpan<&'a str>, _: ErrorKind, other: Self) -> Self {
        other
    }
}

impl<'a> nom::error::ParseError<Tokens<'a>> for ParseError<'a> {
    fn from_error_kind(s: Tokens<'a>, kind: ErrorKind) -> Self {
        ParseError::Parse(s, kind)
    }

    fn append(_: Tokens<'a>, _: ErrorKind, other: Self) -> Self {
        other
    }
}
