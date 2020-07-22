use super::*;
use crate::*;
use nom::{bytes::complete::tag, combinator::map, sequence::tuple};

pub(super) fn static_node(s: Span) -> Result<StaticNode> {
  map(
    tuple((
      static_ident,
      ignore_token0,
      static_type,
      static_value,
      ignore_token0,
      semicolon,
    )),
    |(ident, _, ty, value, _, _)| StaticNode { ident, ty, value },
  )(s)
}

fn static_ident(s: Span) -> Result<Positioned<IdentNode>> {
  map(
    tuple((tag("static"), ignore_token1, positioned(ident_node))),
    |(_, _, ident)| ident,
  )(s)
}

fn static_type(s: Span) -> Result<Positioned<TypeNode>> {
  map(
    tuple((colon, ignore_token0, positioned(type_node))),
    |(_, _, ty)| ty,
  )(s)
}

fn static_value(s: Span) -> Result<Positioned<ValueNode>> {
  map(
    tuple((ignore_token0, equal, ignore_token0, positioned(value_node))),
    |(_, _, _, value)| value,
  )(s)
}
