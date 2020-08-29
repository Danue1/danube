use super::*;

pub(super) fn parse_immutablity_kind(s: Tokens) -> ParseResult<ImmutablityKind> {
  map(opt(parse_keyword(Keyword::Mut)), |keyword| {
    if keyword.is_some() {
      ImmutablityKind::Nope
    } else {
      ImmutablityKind::Yes
    }
  })(s)
}
