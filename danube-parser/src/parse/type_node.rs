use super::*;

pub(super) fn parse_type_node(s: Tokens) -> ParseResult<TypeNode> {
  let (s, immutablitity) = parse_immutablity(s)?;
  if let Ok((s, node)) = parse_type_array_node(s.clone()) {
    Ok((s, TypeNode::Array(immutablitity, node)))
  } else if let Ok((s, node)) = parse_type_tuple_node(s.clone()) {
    Ok((s, TypeNode::Tuple(immutablitity, node)))
  } else if let Ok((s, node)) = parse_path_node(s.clone()) {
    Ok((s, TypeNode::Path(immutablitity, node)))
  } else {
    Err(nom::Err::Error(nom::error::make_error(
      s,
      nom::error::ErrorKind::Alt,
    )))
  }
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
