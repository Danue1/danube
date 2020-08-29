use super::*;

pub(super) fn parse_use_extra_kind(s: Tokens) -> ParseResult<UseExtraKind> {
  alt((parse_use_extra_all, parse_use_extra_sub))(s)
}

fn parse_use_extra_all(s: Tokens) -> ParseResult<UseExtraKind> {
  map(parse_symbol(Symbol::Mul), |_| UseExtraKind::All)(s)
}

fn parse_use_extra_sub(s: Tokens) -> ParseResult<UseExtraKind> {
  let (s, ident) = parse_ident_node(s)?;

  if let Ok((s, alias)) = parse_use_extra_ident(s.clone()) {
    Ok((s, UseExtraKind::Ident(ident, Some(alias))))
  } else if let Ok((s, extra)) = parse_use_kind(parse_use_extra_double_colon)(s.clone()) {
    Ok((s, UseExtraKind::Extra(ident, Box::new(extra))))
  } else {
    Ok((s, UseExtraKind::Ident(ident, None)))
  }
}

fn parse_use_extra_ident(s: Tokens) -> ParseResult<IdentNode> {
  map(
    tuple((parse_keyword(Keyword::As), parse_ident_node)),
    |(_, ident)| ident,
  )(s)
}

fn parse_use_extra_double_colon(s: Tokens) -> ParseResult<UseExtraKind> {
  map(
    tuple((parse_symbol(Symbol::DoubleColon), parse_use_extra_kind)),
    |(_, ident)| ident,
  )(s)
}
