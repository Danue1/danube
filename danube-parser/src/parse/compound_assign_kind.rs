use super::*;

pub(super) fn parse_compound_assign_kind(s: Tokens) -> ParseResult<CompoundAssignKind> {
  alt((
    map(parse_symbol(Symbol::AddAssign), |_| {
      CompoundAssignKind::AddAssign
    }),
    map(parse_symbol(Symbol::SubAssign), |_| {
      CompoundAssignKind::SubAssign
    }),
    map(parse_symbol(Symbol::ExpAssign), |_| {
      CompoundAssignKind::ExpAssign
    }),
    map(parse_symbol(Symbol::MulAssign), |_| {
      CompoundAssignKind::MulAssign
    }),
    map(parse_symbol(Symbol::DivAssign), |_| {
      CompoundAssignKind::DivAssign
    }),
    map(parse_symbol(Symbol::ModAssign), |_| {
      CompoundAssignKind::ModAssign
    }),
    map(parse_symbol(Symbol::AndAssign), |_| {
      CompoundAssignKind::AndAssign
    }),
    map(parse_symbol(Symbol::OrAssign), |_| {
      CompoundAssignKind::OrAssign
    }),
    map(parse_symbol(Symbol::BitAndAssign), |_| {
      CompoundAssignKind::BitAndAssign
    }),
    map(parse_symbol(Symbol::BitOrAssign), |_| {
      CompoundAssignKind::BitOrAssign
    }),
    map(parse_symbol(Symbol::BitXorAssign), |_| {
      CompoundAssignKind::BitXorAssign
    }),
    map(parse_symbol(Symbol::BitLeftAssign), |_| {
      CompoundAssignKind::BitLeftAssign
    }),
    map(parse_symbol(Symbol::BitRightAssign), |_| {
      CompoundAssignKind::BitRightAssign
    }),
  ))(s)
}
