use super::*;

pub(super) fn parse_implement_trait_node(t: Tokens) -> ParseResult<ImplementTraitNode> {
    map(
        tuple((
            parse_visibility_kind,
            parse_keyword(Keyword::Impl),
            parse_path_node,
            parse_generic_node_list,
            parse_keyword(Keyword::For),
            parse_path_node,
            parse_generic_node_list,
            parse_symbol(Symbol::LeftBrace),
            many0(parse_implement_item_kind),
            parse_symbol(Symbol::RightBrace),
        )),
        |(
            visibility,
            _,
            trait_ident,
            generic_list,
            _,
            target,
            target_generic_list,
            _,
            item_list,
            _,
        )| {
            ImplementTraitNode {
                visibility,
                target,
                target_generic_list,
                trait_ident,
                generic_list,
                item_list,
            }
        },
    )(t)
}
