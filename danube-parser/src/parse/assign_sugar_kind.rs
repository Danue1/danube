use super::*;

pub(super) fn parse_assign_sugar_kind(s: Tokens) -> ParseResult<AssignSugarKind> {
  alt((
    map(parse_symbol(Symbol::AddAssign), |_| {
      AssignSugarKind::AddAssign
    }),
    map(parse_symbol(Symbol::SubAssign), |_| {
      AssignSugarKind::SubAssign
    }),
    map(parse_symbol(Symbol::ExpAssign), |_| {
      AssignSugarKind::ExpAssign
    }),
    map(parse_symbol(Symbol::MulAssign), |_| {
      AssignSugarKind::MulAssign
    }),
    map(parse_symbol(Symbol::DivAssign), |_| {
      AssignSugarKind::DivAssign
    }),
    map(parse_symbol(Symbol::ModAssign), |_| {
      AssignSugarKind::ModAssign
    }),
    map(parse_symbol(Symbol::AndAssign), |_| {
      AssignSugarKind::AndAssign
    }),
    map(parse_symbol(Symbol::OrAssign), |_| {
      AssignSugarKind::OrAssign
    }),
    map(parse_symbol(Symbol::BitAndAssign), |_| {
      AssignSugarKind::BitAndAssign
    }),
    map(parse_symbol(Symbol::BitOrAssign), |_| {
      AssignSugarKind::BitOrAssign
    }),
    map(parse_symbol(Symbol::BitXorAssign), |_| {
      AssignSugarKind::BitXorAssign
    }),
    map(parse_symbol(Symbol::BitLeftAssign), |_| {
      AssignSugarKind::BitLeftAssign
    }),
    map(parse_symbol(Symbol::BitRightAssign), |_| {
      AssignSugarKind::BitRightAssign
    }),
  ))(s)
}
