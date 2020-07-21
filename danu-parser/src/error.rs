use crate::{Result, Span};
use nom::error::{ErrorKind, ParseError};

#[derive(Debug, PartialEq)]
pub enum Error<'a> {
  Nom(Span<'a>, ErrorKind),
}

impl<'a> ParseError<Span<'a>> for Error<'a> {
  fn from_error_kind(s: Span<'a>, kind: ErrorKind) -> Self {
    Error::Nom(s, kind)
  }

  fn append(_: Span<'a>, _: ErrorKind, other: Self) -> Self {
    other
  }
}

#[allow(dead_code)]
pub(crate) fn err<'a, O, F, E>(f: F, e: E) -> impl Fn(Span<'a>) -> Result<O>
where
  F: Fn(Span<'a>) -> Result<O>,
  E: Fn(Span<'a>, ErrorKind) -> Error,
{
  move |s: Span<'a>| {
    f(s).map_err(|err| {
      if let nom::Err::Error(Error::Nom(span, error_kind)) = err {
        nom::Err::Error(e(span, error_kind))
      } else {
        err
      }
    })
  }
}
