use super::*;

pub(super) fn parse_infix_operator_kind(s: Tokens) -> ParseResult<InfixOperatorKind> {
  alt((
    map(parse_symbol(Symbol::Add), |_| InfixOperatorKind::Add),
    map(parse_symbol(Symbol::Sub), |_| InfixOperatorKind::Sub),
    map(parse_symbol(Symbol::Mul), |_| InfixOperatorKind::Mul),
    map(parse_symbol(Symbol::Div), |_| InfixOperatorKind::Div),
    map(parse_symbol(Symbol::Mod), |_| InfixOperatorKind::Mod),
    map(parse_symbol(Symbol::BitAnd), |_| InfixOperatorKind::BitAnd),
    map(parse_symbol(Symbol::BitOr), |_| InfixOperatorKind::BitOr),
    map(parse_symbol(Symbol::BitXor), |_| InfixOperatorKind::BitXor),
    map(parse_symbol(Symbol::BitLeft), |_| {
      InfixOperatorKind::BitLeft
    }),
    map(parse_symbol(Symbol::BitRight), |_| {
      InfixOperatorKind::BitRight
    }),
    map(parse_symbol(Symbol::Equal), |_| InfixOperatorKind::Equal),
    map(parse_symbol(Symbol::NotEqual), |_| {
      InfixOperatorKind::NotEqual
    }),
    map(parse_symbol(Symbol::LessThan), |_| {
      InfixOperatorKind::LessThan
    }),
    map(parse_symbol(Symbol::LessThanOrEqual), |_| {
      InfixOperatorKind::LessThanOrEqual
    }),
    map(parse_symbol(Symbol::GreaterThan), |_| {
      InfixOperatorKind::GreaterThan
    }),
    map(parse_symbol(Symbol::GreaterThanOrEqual), |_| {
      InfixOperatorKind::GreaterThanOrEqual
    }),
    map(parse_symbol(Symbol::And), |_| InfixOperatorKind::And),
    map(parse_symbol(Symbol::Or), |_| InfixOperatorKind::Or),
    map(parse_symbol(Symbol::ChainArrow), |_| {
      InfixOperatorKind::ChainArrow
    }),
  ))(s)
}
