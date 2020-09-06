use super::*;

pub(super) fn parse_generic_node(s: Tokens) -> ParseResult<GenericNode> {
  map(
    tuple((
      parse_symbol(Symbol::LessThan),
      parse_path_node,
      opt(map(
        tuple((
          parse_symbol(Symbol::Colon),
          separated_nonempty_list(parse_symbol(Symbol::Add), parse_path_node),
        )),
        |(_, path)| path,
      )),
      opt(preceded(
        parse_symbol(Symbol::Assign),
        separated_nonempty_list(parse_symbol(Symbol::Add), parse_path_node),
      )),
      parse_symbol(Symbol::GreaterThan),
    )),
    |(_, path, trait_list, default_trait_list, _)| GenericNode {
      path,
      trait_list: trait_list.unwrap_or_else(Vec::new),
      default_trait_list: default_trait_list.unwrap_or_else(Vec::new),
    },
  )(s)
}
