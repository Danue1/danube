use super::*;

pub(super) fn parse_struct_unnamed_fields_node(s: Tokens) -> ParseResult<StructUnnamedFieldsNode> {
  map(
    tuple((
      parse_symbol(Symbol::LeftParens),
      separated_nonempty_list(parse_symbol(Symbol::Comma), parse_type_node),
      opt(parse_symbol(Symbol::Comma)),
      parse_symbol(Symbol::RightParens),
      parse_symbol(Symbol::Semicolon),
    )),
    |(_, node_list, _, _, _)| StructUnnamedFieldsNode { node_list },
  )(s)
}
