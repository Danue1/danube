use super::*;

pub(super) fn parse_function_argument_node(s: Tokens) -> ParseResult<FunctionArgumentNode> {
  map(
    tuple((
      parse_ident_node,
      parse_symbol(Symbol::Colon),
      parse_type_node,
    )),
    |(ident, _, ty)| FunctionArgumentNode { ident, ty },
  )(s)
}
