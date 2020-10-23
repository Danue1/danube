use super::*;

pub(super) fn parse_implement_node(t: Tokens) -> ParseResult<ImplementNode> {
    map(
        tuple((
            parse_visibility_kind,
            parse_keyword(Keyword::Impl),
            parse_path_node,
            parse_generic_node_list,
            parse_symbol(Symbol::LeftBrace),
            many0(parse_implement_item_kind),
            parse_symbol(Symbol::RightBrace),
        )),
        |(visibility, _, target, generic_list, _, item_list, _)| ImplementNode {
            visibility,
            target,
            generic_list,
            item_list,
        },
    )(t)
}
