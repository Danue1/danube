use crate::{Result, Span};
use nom::{combinator::map, sequence::tuple};

#[derive(Debug, PartialEq)]
pub struct Position {
  line: usize,
  column: usize,
  offset: usize,
}

#[derive(Debug, PartialEq)]
pub struct Positioned<T: Sized> {
  start: Position,
  end: Position,
  node: T,
}

pub(crate) fn position(s: Span) -> Result<Position> {
  map(nom_locate::position, |s: Span| Position {
    line: s.location_line() as usize,
    column: s.get_column(),
    offset: s.location_offset() as usize,
  })(s)
}

pub(crate) fn positioned<O, F>(f: F) -> impl Fn(Span) -> Result<Positioned<O>>
where
  F: Copy + Fn(Span) -> Result<O>,
{
  move |s: Span| {
    map(tuple((position, f, position)), |(start, node, end)| {
      Positioned { start, end, node }
    })(s)
  }
}
