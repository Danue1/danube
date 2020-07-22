use super::*;
use crate::*;
use nom::combinator::map;

pub(super) fn expression_node(s: Span) -> Result<ExpressionNode> {
  alt((
    map(value_node, ExpressionNode::Value),
    map(if_node, ExpressionNode::If),
  ))(s)
}
