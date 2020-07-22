use super::*;
use crate::*;
use nom::combinator::map;

pub(super) fn pattern_node(s: Span) -> Result<PatternNode> {
  map(pattern_node_literal, PatternNode::Literal)(s)
}

fn pattern_node_literal(s: Span) -> Result<LiteralValueNode> {
  literal_value_node(s)
}
