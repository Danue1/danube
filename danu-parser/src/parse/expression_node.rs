use super::*;
use crate::*;
use nom::combinator::map;

pub(super) fn expression_node(s: Span) -> Result<ExpressionNode> {
  alt((
    map(literal_value_node, ExpressionNode::Literal),
    map(expression_conditional_node, ExpressionNode::Conditional),
    map(loop_node, ExpressionNode::Loop),
    map(while_node, ExpressionNode::While),
    map(pattern_match_node, ExpressionNode::PatternMatch),
  ))(s)
}
