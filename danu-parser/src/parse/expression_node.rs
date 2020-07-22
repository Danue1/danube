use super::*;
use crate::*;
use nom::combinator::map;

pub(super) fn expression_node(s: Span) -> Result<ExpressionNode> {
  alt((
    map(value_node, ExpressionNode::Value),
    map(expression_conditional_node, ExpressionNode::Conditional),
    map(loop_node, ExpressionNode::Loop),
    map(while_node, ExpressionNode::While),
  ))(s)
}
