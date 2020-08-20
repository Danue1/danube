use super::*;

pub(super) fn parse_arithmetic_or_logical_kind(s: Tokens) -> ParseResult<ArithmeticOrLogicalKind> {
  alt((
    map(parse_symbol(Symbol::Add), |_| ArithmeticOrLogicalKind::Add),
    map(parse_symbol(Symbol::Sub), |_| ArithmeticOrLogicalKind::Sub),
    map(parse_symbol(Symbol::Mul), |_| ArithmeticOrLogicalKind::Mul),
    map(parse_symbol(Symbol::Div), |_| ArithmeticOrLogicalKind::Div),
    map(parse_symbol(Symbol::Mod), |_| ArithmeticOrLogicalKind::Mod),
    map(parse_symbol(Symbol::BitAnd), |_| {
      ArithmeticOrLogicalKind::BitAnd
    }),
    map(parse_symbol(Symbol::BitOr), |_| {
      ArithmeticOrLogicalKind::BitOr
    }),
    map(parse_symbol(Symbol::BitXor), |_| {
      ArithmeticOrLogicalKind::BitXor
    }),
    map(parse_symbol(Symbol::BitLeft), |_| {
      ArithmeticOrLogicalKind::BitLeft
    }),
    map(parse_symbol(Symbol::BitRight), |_| {
      ArithmeticOrLogicalKind::BitRight
    }),
  ))(s)
}
