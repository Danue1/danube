use super::*;

pub fn parse_type_kind(t: Tokens) -> ParseResult<TypeKind> {
    map(parse_path_node, TypeKind::Path)(t)
}
