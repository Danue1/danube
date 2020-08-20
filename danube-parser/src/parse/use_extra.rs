use super::*;

pub(super) fn parse_use_extra(s: Tokens) -> ParseResult<UseExtra> {
  alt((parse_use_extra_all, parse_use_extra_sub))(s)
}

fn parse_use_extra_all(s: Tokens) -> ParseResult<UseExtra> {
  map(parse_symbol(Symbol::Mul), |_| UseExtra::All)(s)
}

fn parse_use_extra_sub(s: Tokens) -> ParseResult<UseExtra> {
  let (s, ident) = parse_ident_node(s)?;

  if let Ok((s, alias)) = parse_use_extra_ident(s.clone()) {
    Ok((s, UseExtra::Ident(ident, Some(alias))))
  } else if let Ok((s, extra)) = parse_use_kind(parse_use_extra_double_colon)(s.clone()) {
    Ok((s, UseExtra::Extra(ident, Box::new(extra))))
  } else {
    Ok((s, UseExtra::Ident(ident, None)))
  }
}

fn parse_use_extra_ident(s: Tokens) -> ParseResult<IdentNode> {
  map(
    tuple((parse_keyword(Keyword::As), parse_ident_node)),
    |(_, ident)| ident,
  )(s)
}

fn parse_use_extra_double_colon(s: Tokens) -> ParseResult<UseExtra> {
  map(
    tuple((parse_symbol(Symbol::DoubleColon), parse_use_extra)),
    |(_, ident)| ident,
  )(s)
}
