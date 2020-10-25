use super::*;

pub(super) fn parse_generic_node_list(a: Tokens) -> ParseResult<GenericNodeList> {
    map(
        opt(tuple((
            parse_symbol(Symbol::LeftBracket),
            separated_nonempty_list(parse_symbol(Symbol::Comma), parse_generic_node),
            opt(parse_symbol(Symbol::Comma)),
            parse_symbol(Symbol::RightBracket),
        ))),
        |generic_list| {
            generic_list
                .map(|(_, node_list, _, _)| node_list)
                .unwrap_or_default()
        },
    )(a)
}
