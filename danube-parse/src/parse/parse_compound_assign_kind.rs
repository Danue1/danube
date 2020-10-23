use super::*;

pub(super) fn parse_compound_assign_kind(t: Tokens) -> ParseResult<CompoundAssignKind> {
    alt((
        map(parse_symbol(Symbol::AddAssign), |_| CompoundAssignKind::Add),
        map(parse_symbol(Symbol::SubAssign), |_| CompoundAssignKind::Sub),
        map(parse_symbol(Symbol::ExpAssign), |_| CompoundAssignKind::Exp),
        map(parse_symbol(Symbol::MulAssign), |_| CompoundAssignKind::Mul),
        map(parse_symbol(Symbol::DivAssign), |_| CompoundAssignKind::Div),
        map(parse_symbol(Symbol::ModAssign), |_| CompoundAssignKind::Mod),
        map(parse_symbol(Symbol::AndAssign), |_| CompoundAssignKind::And),
        map(parse_symbol(Symbol::OrAssign), |_| CompoundAssignKind::Or),
        map(parse_symbol(Symbol::BitAndAssign), |_| {
            CompoundAssignKind::BitAnd
        }),
        map(parse_symbol(Symbol::BitOrAssign), |_| {
            CompoundAssignKind::BitOr
        }),
        map(parse_symbol(Symbol::BitXorAssign), |_| {
            CompoundAssignKind::BitXor
        }),
        map(parse_symbol(Symbol::BitLeftAssign), |_| {
            CompoundAssignKind::BitLeft
        }),
        map(parse_symbol(Symbol::BitRightAssign), |_| {
            CompoundAssignKind::BitRight
        }),
    ))(t)
}
