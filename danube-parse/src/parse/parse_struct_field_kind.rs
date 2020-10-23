use super::*;

pub(super) fn parse_struct_field_kind(t: Tokens) -> ParseResult<StructFieldKind> {
    alt((
        map(parse_struct_unnamed_field_node, StructFieldKind::Unnamed),
        map(parse_struct_named_field_node, StructFieldKind::Named),
    ))(t)
}
