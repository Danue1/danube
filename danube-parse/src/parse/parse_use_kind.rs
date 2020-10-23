use super::*;

pub(super) fn parse_use_kind<O, F>(f: F) -> impl Fn(Tokens) -> ParseResult<UseKind<O>>
where
    F: Copy + Fn(Tokens) -> ParseResult<O>,
{
    move |s: Tokens| {
        alt((
            map(f, UseKind::Unnested),
            map(
                tuple((
                    parse_symbol(Symbol::LeftBrace),
                    separated_nonempty_list(parse_symbol(Symbol::Comma), f),
                    opt(parse_symbol(Symbol::Comma)),
                    parse_symbol(Symbol::RightBrace),
                )),
                |(_, node_list, _, _)| UseKind::Nested(node_list),
            ),
        ))(s)
    }
}
