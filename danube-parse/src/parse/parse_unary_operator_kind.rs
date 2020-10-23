use super::*;

pub(super) fn parse_unary_operator_kind(t: Tokens) -> ParseResult<UnaryOperatorKind> {
    alt((
        map(parse_symbol(Symbol::Not), |_| UnaryOperatorKind::Not),
        map(parse_symbol(Symbol::Sub), |_| UnaryOperatorKind::Negative),
    ))(t)
}
