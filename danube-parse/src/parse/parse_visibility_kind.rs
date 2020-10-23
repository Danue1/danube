use super::*;

pub(super) fn parse_visibility_kind(t: Tokens) -> ParseResult<VisibilityKind> {
    let (s, _) = parse_keyword(Keyword::Public)(t)?;

    match tuple((
        parse_symbol(Symbol::LeftParens),
        parse_visibility,
        parse_symbol(Symbol::RightParens),
    ))(s.clone())
    {
        Ok((s, (_, Visibility::Super, _))) => Ok((s, VisibilityKind::Super)),
        Ok((s, (_, Visibility::Module, _))) => Ok((s, VisibilityKind::Module)),
        Ok((s, (_, Visibility::Restricted(path), _))) => Ok((s, VisibilityKind::Restricted(path))),
        _ => Ok((s, VisibilityKind::Public)),
    }
}

enum Visibility {
    Super,
    Module,
    Restricted(PathNode),
}

fn parse_visibility(t: Tokens) -> ParseResult<Visibility> {
    alt((
        map(parse_keyword(Keyword::Super), |_| Visibility::Super),
        map(parse_keyword(Keyword::Module), |_| Visibility::Module),
        map(
            tuple((parse_keyword(Keyword::In), parse_path_node)),
            |(_, path)| Visibility::Restricted(path),
        ),
    ))(t)
}
