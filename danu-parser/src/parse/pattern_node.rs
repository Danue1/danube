use super::*;
use crate::*;
use nom::{bytes::complete::tag, combinator::map, multi::separated_list};

pub(super) fn pattern_node(s: Span) -> Result<PatternNode> {
  alt((
    map(pattern_node_literal, PatternNode::Literal),
    map(pattern_node_path, PatternNode::Path),
  ))(s)
}

fn pattern_node_literal(s: Span) -> Result<LiteralValueNode> {
  literal_value_node(s)
}

fn pattern_node_path(s: Span) -> Result<PathNode> {
  map(
    separated_list(tuple((ignore_token0, tag("::"), ignore_token0)), ident_node),
    |ident_list| PathNode { ident_list },
  )(s)
}
