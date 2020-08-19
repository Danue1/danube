use super::*;

pub(super) fn parse_immutablity(s: Tokens) -> ParseResult<Immutablity> {
  map(opt(parse_keyword(Keyword::Mut)), |keyword| {
    if keyword.is_some() {
      Immutablity::Nope
    } else {
      Immutablity::Yes
    }
  })(s)
}
