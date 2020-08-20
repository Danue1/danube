use super::*;

pub(super) fn parse_named_struct_node(s: Tokens) -> ParseResult<NamedStructNode> {
  map(
    tuple((
      opt(parse_path_node),
      parse_symbol(Symbol::LeftBrace),
      separated_nonempty_list(parse_symbol(Symbol::Comma), parse_field_node),
      parse_symbol(Symbol::RightBrace),
    )),
    |(path, _, field_list, _)| NamedStructNode { path, field_list },
  )(s)
}
