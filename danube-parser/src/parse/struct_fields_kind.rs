use super::*;

pub(super) fn parse_struct_fields_kind(s: Tokens) -> ParseResult<StructFieldsKind> {
  alt((
    map(parse_struct_named_fields_node, StructFieldsKind::Named),
    map(parse_struct_unnamed_fields_node, StructFieldsKind::Unnamed),
  ))(s)
}
