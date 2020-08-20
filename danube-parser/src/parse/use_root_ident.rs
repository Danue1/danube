use super::*;

pub(super) fn parse_use_root_ident(s: Tokens) -> ParseResult<UseRootIdent> {
  alt((
    map(parse_keyword(Keyword::VariableSelf), |_| {
      UseRootIdent::Current
    }),
    map(parse_keyword(Keyword::Super), |_| UseRootIdent::Super),
    map(parse_keyword(Keyword::Module), |_| UseRootIdent::Module),
    map(parse_ident_node, UseRootIdent::Ident),
  ))(s)
}
