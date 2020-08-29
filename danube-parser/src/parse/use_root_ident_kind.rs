use super::*;

pub(super) fn parse_use_root_ident_kind(s: Tokens) -> ParseResult<UseRootIdentKind> {
  alt((
    map(parse_keyword(Keyword::VariableSelf), |_| {
      UseRootIdentKind::Current
    }),
    map(parse_keyword(Keyword::Super), |_| UseRootIdentKind::Super),
    map(parse_keyword(Keyword::Module), |_| UseRootIdentKind::Module),
    map(parse_ident_node, UseRootIdentKind::Ident),
  ))(s)
}
