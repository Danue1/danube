use super::*;

pub(super) fn parse_function_argument_node(s: Tokens) -> ParseResult<FunctionArgumentNode> {
  map(
    tuple((
      parse_immutablity_kind,
      parse_ident_node,
      parse_symbol(Symbol::Colon),
      parse_type_kind,
    )),
    |(immutablity, ident, _, ty)| FunctionArgumentNode {
      ident,
      immutablity,
      ty,
    },
  )(s)
}
