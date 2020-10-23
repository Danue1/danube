use super::*;

pub(super) fn parse_enum_variant_kind(t: Tokens) -> ParseResult<EnumVariantKind> {
    alt((
        map(parse_enum_unnamed_variant_node, EnumVariantKind::Unnamed),
        map(parse_enum_named_variant_node, EnumVariantKind::Named),
    ))(t)
}
