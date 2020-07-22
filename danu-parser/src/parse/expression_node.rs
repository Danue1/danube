use super::*;
use crate::*;
use nom::combinator::map;

pub(super) fn expression_node(s: Span) -> Result<ExpressionNode> {
  map(value_node, ExpressionNode::Value)(s)
}
