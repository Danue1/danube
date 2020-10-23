use super::*;

pub(super) fn parse_immutablity_kind(t: Tokens) -> ParseResult<ImmutablityKind> {
    map(opt(parse_keyword(Keyword::Mut)), |immutablity| {
        immutablity
            .map(|_| ImmutablityKind::Nope)
            .unwrap_or(ImmutablityKind::Yes)
    })(t)
}
