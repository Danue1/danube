use super::*;

pub(super) fn parse_generic_node_list(t: Tokens) -> ParseResult<GenericNodeList> {
    map(
        tuple((
            parse_symbol(Symbol::LeftBracket),
            separated_nonempty_list(parse_symbol(Symbol::Comma), parse_generic_node),
            opt(parse_symbol(Symbol::Comma)),
            parse_symbol(Symbol::RightBracket),
        )),
        |(_, node_list, _, _)| node_list,
    )(t)
}
