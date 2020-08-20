use crate::*;
use nom::error::{ErrorKind, ParseError};

#[derive(Debug, PartialEq)]
pub enum Error<'a> {
  Nom(nom_locate::LocatedSpan<&'a str>, ErrorKind),
  Lex(&'a str),
  Parse(Tokens<'a>, ErrorKind),
  EOF,
}

impl<'a> ParseError<nom_locate::LocatedSpan<&'a str>> for Error<'a> {
  fn from_error_kind(s: nom_locate::LocatedSpan<&'a str>, kind: ErrorKind) -> Self {
    Error::Nom(s, kind)
  }

  fn append(_: nom_locate::LocatedSpan<&'a str>, _: ErrorKind, other: Self) -> Self {
    other
  }
}

impl<'a> ParseError<Tokens<'a>> for Error<'a> {
  fn from_error_kind(s: Tokens<'a>, kind: ErrorKind) -> Self {
    Error::Parse(s, kind)
  }

  fn append(_: Tokens<'a>, _: ErrorKind, other: Self) -> Self {
    other
  }
}
