use super::*;

pub(super) fn parse_value_kind(t: Tokens) -> ParseResult<ValueKind> {
    alt((
        map(parse_literal_kind, ValueKind::Literal),
        map(parse_path_node, ValueKind::Path),
    ))(t)
}
