use super::*;

pub(super) fn parse_trait_item_kind(s: Tokens) -> ParseResult<TraitItemKind> {
  alt((
    map(parse_output_type_node, TraitItemKind::OutputType),
    map(parse_trait_item_constant_node, TraitItemKind::Constant),
    map(parse_trait_item_function_node, TraitItemKind::Function),
  ))(s)
}
