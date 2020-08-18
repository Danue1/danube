use super::*;

pub(super) fn parse_statement_node(s: Tokens) -> ParseResult<StatementNode> {
  alt((
    map(parse_constant_node, StatementNode::Constant),
    map(parse_static_node, StatementNode::Static),
    map(parse_let_node, StatementNode::Let),
    map(parse_let_mut_node, StatementNode::LetMut),
    map(parse_statement_conditional_node, StatementNode::Conditional),
    map(parse_loop_node, StatementNode::Loop),
    map(parse_while_node, StatementNode::While),
    map(parse_pattern_match_node, StatementNode::PatternMatch),
    map(parse_expression_node, StatementNode::Expression),
  ))(s)
}
