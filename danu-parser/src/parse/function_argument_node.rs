use super::*;

pub(super) fn parse_function_argument_node(s: Tokens) -> ParseResult<FunctionArgumentNode> {
  map(
    tuple((
      opt(parse_keyword(Keyword::Mut)),
      parse_ident_node,
      parse_symbol(Symbol::Colon),
      parse_type_node,
    )),
    |(is_mutable, ident, _, ty)| FunctionArgumentNode {
      ident,
      is_mutable: is_mutable.is_some(),
      ty,
    },
  )(s)
}
