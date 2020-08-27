use super::*;

pub(super) fn parse_item_node(s: Tokens) -> ParseResult<ItemNode> {
  alt((
    map(parse_use_node, ItemNode::Use),
    map(parse_struct_node, ItemNode::Struct),
    map(parse_enum_node, ItemNode::Enum),
    map(parse_function_node, ItemNode::Function),
    map(parse_type_alias_node, ItemNode::TypeAlias),
    map(parse_trait_node, ItemNode::Trait),
    map(parse_constant_node, ItemNode::Constant),
    map(parse_static_node, ItemNode::Static),
    map(parse_implement_node, ItemNode::Implement),
    map(parse_implement_trait_node, ItemNode::ImplementTrait),
  ))(s)
}
