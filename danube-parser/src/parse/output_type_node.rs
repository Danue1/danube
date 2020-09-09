use super::*;

pub(super) fn parse_output_type_node(s: Tokens) -> ParseResult<TraitItemOutputTypeNode> {
  map(
    tuple((
      parse_keyword(Keyword::Type),
      parse_ident_node,
      opt(preceded(parse_symbol(Symbol::Assign), parse_type_kind)),
      parse_symbol(Symbol::Semicolon),
    )),
    |(_, ident, ty, _)| TraitItemOutputTypeNode { ident, ty },
  )(s)
}
