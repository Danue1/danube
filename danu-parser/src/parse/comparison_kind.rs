use super::*;

pub(super) fn parse_comparison_kind(s: Tokens) -> ParseResult<ComparisonKind> {
  alt((
    map(parse_symbol(Symbol::Equal), |_| ComparisonKind::Equal),
    map(parse_symbol(Symbol::NotEqual), |_| ComparisonKind::NotEqual),
    map(parse_symbol(Symbol::LessThan), |_| ComparisonKind::LessThan),
    map(parse_symbol(Symbol::LessThanOrEqual), |_| {
      ComparisonKind::LessThanOrEqual
    }),
    map(parse_symbol(Symbol::GreaterThan), |_| {
      ComparisonKind::GreaterThan
    }),
    map(parse_symbol(Symbol::GreaterThanOrEqual), |_| {
      ComparisonKind::GreaterThanOrEqual
    }),
  ))(s)
}
