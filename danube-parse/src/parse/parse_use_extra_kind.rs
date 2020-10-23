use super::*;

pub(super) fn parse_use_extra_kind(t: Tokens) -> ParseResult<UseExtraKind> {
    alt((parse_use_extra_all, parse_use_extra_sub))(t)
}

fn parse_use_extra_all(t: Tokens) -> ParseResult<UseExtraKind> {
    map(parse_symbol(Symbol::Mul), |_| UseExtraKind::All)(t)
}

fn parse_use_extra_sub(t: Tokens) -> ParseResult<UseExtraKind> {
    let (s, ident) = parse_ident_node(t)?;

    if let Ok((s, alias)) = parse_use_extra_ident(s.clone()) {
        Ok((s, UseExtraKind::Ident(ident, Some(alias))))
    } else if let Ok((s, extra)) = parse_use_kind(parse_use_extra_double_colon)(s.clone()) {
        Ok((s, UseExtraKind::Extra(ident, Box::new(extra))))
    } else {
        Ok((s, UseExtraKind::Ident(ident, None)))
    }
}

fn parse_use_extra_ident(t: Tokens) -> ParseResult<IdentNode> {
    map(
        tuple((parse_keyword(Keyword::As), parse_ident_node)),
        |(_, ident)| ident,
    )(t)
}

fn parse_use_extra_double_colon(t: Tokens) -> ParseResult<UseExtraKind> {
    map(
        tuple((parse_symbol(Symbol::DoubleColon), parse_use_extra_kind)),
        |(_, ident)| ident,
    )(t)
}
