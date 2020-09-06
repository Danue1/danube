use super::*;

pub(super) fn parse_implement_item_kind(s: Tokens) -> ParseResult<ImplementItemKind> {
  alt((
    map(
      parse_implement_output_type_node,
      ImplementItemKind::OutputType,
    ),
    map(parse_constant_node, ImplementItemKind::Constant),
    map(parse_function_node, ImplementItemKind::Function),
  ))(s)
}
