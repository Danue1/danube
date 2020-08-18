use super::*;

pub(super) fn parse_type_node(s: Tokens) -> ParseResult<TypeNode> {
  alt((
    map(parse_type_array_node, |node| {
      TypeNode::Array(Box::new(node))
    }),
    map(parse_type_tuple_node, TypeNode::Tuple),
    map(parse_ident_node, TypeNode::Ident),
  ))(s)
}

fn parse_type_tuple_node(s: Tokens) -> ParseResult<Vec<TypeNode>> {
  map(
    tuple((
      parse_symbol(Symbol::LeftParens),
      separated_nonempty_list(parse_symbol(Symbol::Comma), parse_type_node),
      opt(parse_symbol(Symbol::Comma)),
      parse_symbol(Symbol::RightParens),
    )),
    |(_, node_list, _, _)| node_list,
  )(s)
}
