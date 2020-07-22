use super::*;
use crate::*;
use nom::combinator::map;

pub(super) fn pattern_node(s: Span) -> Result<PatternNode> {
  map(pattern_node_value, PatternNode::Value)(s)
}

fn pattern_node_value(s: Span) -> Result<ValueNode> {
  value_node(s)
}
