use super::*;

pub(super) fn parse_type_kind(s: Tokens) -> ParseResult<TypeKind> {
  let (s, immutablitity) = parse_immutablity_kind(s)?;
  if let Ok((s, node)) = parse_type_array_node(s.clone()) {
    Ok((s, TypeKind::Array(immutablitity, node)))
  } else if let Ok((s, node)) = parse_type_tuple_node(s.clone()) {
    Ok((s, TypeKind::Tuple(immutablitity, node)))
  } else {
    let (s, node) = alt((
      map(parse_keyword(Keyword::TypeSelf), |_| PathNode {
        ident_list: vec![IdentNode {
          raw: "Self".to_owned(),
        }],
      }),
      parse_path_node,
    ))(s.clone())?;
    match tuple((
      parse_symbol(Symbol::LessThan),
      separated_nonempty_list(parse_symbol(Symbol::Comma), parse_path_node),
      parse_symbol(Symbol::GreaterThan),
    ))(s.clone())
    {
      Ok((s, (_, path_list, _))) => Ok((s, TypeKind::Generic(immutablitity, node, path_list))),
      _ => Ok((s, TypeKind::Path(immutablitity, node))),
    }
  }
}

fn parse_type_tuple_node(s: Tokens) -> ParseResult<Vec<TypeKind>> {
  map(
    tuple((
      parse_symbol(Symbol::LeftParens),
      separated_list(parse_symbol(Symbol::Comma), parse_type_kind),
      opt(parse_symbol(Symbol::Comma)),
      parse_symbol(Symbol::RightParens),
    )),
    |(_, node_list, _, _)| node_list,
  )(s)
}
