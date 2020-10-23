use super::*;

pub(super) fn parse_literal_kind(t: Tokens) -> ParseResult<LiteralKind> {
    alt((
        map(parse_int, LiteralKind::Int),
        map(parse_float, LiteralKind::Float),
        map(parse_string, LiteralKind::String),
    ))(t)
}
