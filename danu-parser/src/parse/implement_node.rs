use super::*;
use crate::*;
use nom::{bytes::complete::tag, combinator::map, multi::many1, sequence::tuple};

pub(super) fn implement_node(s: Span) -> Result<ImplementNode> {
  map(
    tuple((
      tag("impl"),
      ignore_token1,
      positioned(ident_node),
      ignore_token0,
      left_brace,
      many1(map(
        tuple((ignore_token0, positioned(implement_item_node))),
        |(_, item)| item,
      )),
      ignore_token0,
      right_brace,
    )),
    |(_, _, target, _, _, item_list, _, _)| ImplementNode { target, item_list },
  )(s)
}

fn implement_item_node(s: Span) -> Result<ImplementItemNode> {
  alt((
    map(constant_node, ImplementItemNode::Constant),
    map(function_node, ImplementItemNode::Function),
  ))(s)
}
