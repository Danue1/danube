use super::*;

pub(super) fn parse_statement_node(s: Tokens) -> ParseResult<StatementNode> {
  alt((
    map(parse_constant_node, StatementNode::Constant),
    map(parse_static_node, StatementNode::Static),
    map(parse_assign_sugar_node, StatementNode::AssignSugar),
    map(parse_let_node, StatementNode::Let),
    map(parse_expression_node, StatementNode::Expression),
    map(parse_symbol(Symbol::Semicolon), |_| {
      StatementNode::Semicolon
    }),
  ))(s)
}
