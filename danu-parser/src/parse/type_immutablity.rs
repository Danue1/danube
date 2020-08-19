use super::*;

pub(super) fn parse_type_immutablity(s: Tokens) -> ParseResult<TypeImmutablity> {
  map(opt(parse_keyword(Keyword::Mut)), |keyword| {
    if keyword.is_some() {
      TypeImmutablity::No
    } else {
      TypeImmutablity::Yes
    }
  })(s)
}
