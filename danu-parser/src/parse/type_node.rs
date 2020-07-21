use super::*;
use crate::*;
use nom::{branch::alt, combinator::map, sequence::tuple};

pub(super) fn type_node(s: Span) -> Result<TypeNode> {
  alt((
    map(type_array_node, |node| TypeNode::Array(Box::new(node))),
    map(ident_node, TypeNode::Ident),
  ))(s)
}

fn type_array_node(s: Span) -> Result<TypeArrayNode> {
  map(
    tuple((
      left_bracket,
      ignore_token0,
      positioned(type_node),
      ignore_token0,
      semicolon,
      ignore_token0,
      value_usize,
      ignore_token0,
      right_bracket,
    )),
    |(_, _, ty, _, _, _, size, _, _)| TypeArrayNode { ty, size },
  )(s)
}
