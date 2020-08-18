use super::*;

pub(super) fn parse_type_node(s: Tokens) -> ParseResult<TypeNode> {
  alt((
    map(parse_type_array_node, |node| {
      TypeNode::Array(Box::new(node))
    }),
    map(parse_ident_node, TypeNode::Ident),
  ))(s)
}
