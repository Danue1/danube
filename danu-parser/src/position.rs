use crate::{Result, Span};
use nom::{combinator::map, sequence::tuple};

#[derive(Debug, Clone, PartialEq)]
pub struct Position {
  pub line: usize,
  pub column: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Positioned<T: Sized> {
  pub position: Position,
  pub node: T,
}

pub(crate) fn position(s: Span) -> Result<Position> {
  map(nom_locate::position, |s: Span| Position {
    line: s.location_line() as usize,
    column: s.get_column(),
  })(s)
}

pub(crate) fn positioned<O, F>(f: F) -> impl Fn(Span) -> Result<Positioned<O>>
where
  F: Copy + Fn(Span) -> Result<O>,
{
  move |s: Span| {
    map(tuple((position, f)), |(position, node)| Positioned {
      position,
      node,
    })(s)
  }
}
