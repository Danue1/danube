use super::*;

pub(super) fn parse_enum_variant_node(s: Tokens) -> ParseResult<EnumVariantNode> {
  map(
    tuple((
      parse_ident_node,
      opt(map(
        tuple((
          parse_symbol(Symbol::LeftParens),
          parse_type_kind,
          parse_symbol(Symbol::RightParens),
        )),
        |(_, ident, _)| ident,
      )),
    )),
    |(ident, ty)| EnumVariantNode { ident, ty },
  )(s)
}
