use super::*;

pub(super) fn parse_lazy_boolean_kind(s: Tokens) -> ParseResult<LazyBooleanKind> {
  alt((
    map(parse_symbol(Symbol::And), |_| LazyBooleanKind::And),
    map(parse_symbol(Symbol::Or), |_| LazyBooleanKind::Or),
  ))(s)
}
