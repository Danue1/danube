use super::*;

pub(super) fn parse_struct_node(t: Tokens) -> ParseResult<StructNode> {
    map(
        tuple((
            parse_visibility_kind,
            parse_keyword(Keyword::Struct),
            parse_ident_node,
            parse_generic_node_list,
            parse_struct_field_kind,
        )),
        |(visibility, _, ident, generic_list, fields)| StructNode {
            visibility,
            ident,
            generic_list,
            fields,
        },
    )(t)
}
