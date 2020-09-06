use super::*;

pub(super) fn parse_implement_output_type_node(s: Tokens) -> ParseResult<ImplementOutputTypeNode> {
  map(
    tuple((
      parse_keyword(Keyword::Type),
      parse_ident_node,
      preceded(parse_symbol(Symbol::Assign), parse_type_kind),
      parse_symbol(Symbol::Semicolon),
    )),
    |(_, ident, ty, _)| ImplementOutputTypeNode { ident, ty },
  )(s)
}
