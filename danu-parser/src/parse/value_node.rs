use crate::*;
use nom::{bytes::complete::take_while1, combinator::map};

pub(super) fn value_usize(s: Span) -> Result<usize> {
  map(take_while1(is_digit), |value: Span| {
    value.fragment().parse().unwrap()
  })(s)
}
