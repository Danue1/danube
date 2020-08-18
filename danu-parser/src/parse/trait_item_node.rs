use super::*;

pub(super) fn parse_trait_item_node(s: Tokens) -> ParseResult<TraitItemNode> {
  alt((
    map(parse_trait_item_constant_node, TraitItemNode::Constant),
    map(parse_trait_item_function_node, TraitItemNode::Function),
  ))(s)
}
