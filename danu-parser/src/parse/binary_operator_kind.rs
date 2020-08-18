use super::*;

pub(super) fn parse_binary_operator_kind(s: Tokens) -> ParseResult<BinaryOperatorKind> {
  alt((
    map(parse_symbol(Symbol::Add), |_| BinaryOperatorKind::Add),
    map(parse_symbol(Symbol::Sub), |_| BinaryOperatorKind::Sub),
    map(parse_symbol(Symbol::Mul), |_| BinaryOperatorKind::Mul),
    map(parse_symbol(Symbol::Div), |_| BinaryOperatorKind::Div),
    map(parse_symbol(Symbol::Mod), |_| BinaryOperatorKind::Mod),
    map(parse_symbol(Symbol::And), |_| BinaryOperatorKind::And),
    map(parse_symbol(Symbol::Or), |_| BinaryOperatorKind::Or),
    map(parse_symbol(Symbol::BitXor), |_| BinaryOperatorKind::BitXor),
    map(parse_symbol(Symbol::BitAnd), |_| BinaryOperatorKind::BitAnd),
    map(parse_symbol(Symbol::BitOr), |_| BinaryOperatorKind::BitOr),
    map(parse_symbol(Symbol::BitLeft), |_| {
      BinaryOperatorKind::BitLeft
    }),
    map(parse_symbol(Symbol::BitRight), |_| {
      BinaryOperatorKind::BitRight
    }),
    map(parse_symbol(Symbol::Equal), |_| BinaryOperatorKind::Equal),
    map(parse_symbol(Symbol::NotEqual), |_| {
      BinaryOperatorKind::NotEqual
    }),
    map(parse_symbol(Symbol::LessThan), |_| {
      BinaryOperatorKind::LessThan
    }),
    map(parse_symbol(Symbol::LessThanOrEqual), |_| {
      BinaryOperatorKind::LessThanOrEqual
    }),
    map(parse_symbol(Symbol::GreaterThan), |_| {
      BinaryOperatorKind::GreaterThan
    }),
    map(parse_symbol(Symbol::GreaterThanOrEqual), |_| {
      BinaryOperatorKind::GreaterThanOrEqual
    }),
  ))(s)
}
