use super::*;

pub(super) fn parse_type_kind(s: Tokens) -> ParseResult<TypeKind> {
  let (s, immutablitity) = parse_immutablity_kind(s)?;
  if let Ok((s, node)) = parse_type_array_node(s.clone()) {
    Ok((s, TypeKind::Array(immutablitity, node)))
  } else if let Ok((s, node)) = parse_type_tuple_node(s.clone()) {
    Ok((s, TypeKind::Tuple(immutablitity, node)))
  } else if let Ok((s, node)) = parse_path_node(s.clone()) {
    Ok((s, TypeKind::Path(immutablitity, node)))
  } else {
    Err(nom::Err::Error(nom::error::make_error(
      s,
      nom::error::ErrorKind::Alt,
    )))
  }
}

fn parse_type_tuple_node(s: Tokens) -> ParseResult<Vec<TypeKind>> {
  map(
    tuple((
      parse_symbol(Symbol::LeftParens),
      separated_nonempty_list(parse_symbol(Symbol::Comma), parse_type_kind),
      opt(parse_symbol(Symbol::Comma)),
      parse_symbol(Symbol::RightParens),
    )),
    |(_, node_list, _, _)| node_list,
  )(s)
}
