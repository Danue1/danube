use super::*;

pub(super) fn parse_implement_item_node(s: Tokens) -> ParseResult<ImplementItemNode> {
  alt((
    map(parse_constant_node, ImplementItemNode::Constant),
    map(parse_function_node, ImplementItemNode::Function),
  ))(s)
}
