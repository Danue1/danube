use super::*;
use crate::*;
use nom::{bytes::complete::tag, combinator::map, multi::separated_list, sequence::tuple};

pub(super) fn trait_node(s: Span) -> Result<TraitNode> {
  map(
    tuple((
      tag("trait"),
      ignore_token1,
      positioned(ident_node),
      ignore_token0,
      left_brace,
      ignore_token0,
      separated_list(ignore_token0, positioned(trait_item_node)),
      ignore_token0,
      right_brace,
    )),
    |(_, _, ident, _, _, _, item_list, _, _)| TraitNode { ident, item_list },
  )(s)
}

fn trait_item_node(s: Span) -> Result<TraitItemNode> {
  alt((
    map(trait_item_constant_node, TraitItemNode::Constant),
    map(trait_item_function_node, TraitItemNode::Function),
  ))(s)
}
