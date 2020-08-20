use super::*;

pub(super) fn parse_struct_fields_node(s: Tokens) -> ParseResult<StructFieldsNode> {
  alt((
    map(parse_struct_named_fields_node, StructFieldsNode::Named),
    map(parse_struct_unnamed_fields_node, StructFieldsNode::Unnamed),
  ))(s)
}
