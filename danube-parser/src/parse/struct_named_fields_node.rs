use super::*;

pub(super) fn parse_struct_named_fields_node(s: Tokens) -> ParseResult<StructNamedFieldsNode> {
  map(
    tuple((
      parse_symbol(Symbol::LeftBrace),
      separated_nonempty_list(parse_symbol(Symbol::Comma), parse_node),
      opt(parse_symbol(Symbol::Comma)),
      parse_symbol(Symbol::RightBrace),
    )),
    |(_, node_list, _, _)| StructNamedFieldsNode { node_list },
  )(s)
}

fn parse_node(s: Tokens) -> ParseResult<(IdentNode, TypeNode)> {
  map(
    tuple((
      parse_ident_node,
      parse_symbol(Symbol::Colon),
      parse_type_node,
    )),
    |(ident, _, ty)| (ident, ty),
  )(s)
}
