use super::*;

pub(super) fn parse_trait_node(t: Tokens) -> ParseResult<TraitNode> {
    map(
        tuple((
            parse_visibility_kind,
            parse_keyword(Keyword::Trait),
            parse_ident_node,
            parse_generic_node_list,
            parse_inheritance_list,
            parse_symbol(Symbol::LeftBrace),
            many1(parse_trait_item_kind),
            parse_symbol(Symbol::RightBrace),
        )),
        |(visibility, _, ident, generic_list, inheritance_list, _, item_list, _)| TraitNode {
            visibility,
            ident,
            generic_list,
            inheritance_list,
            item_list,
        },
    )(t)
}

fn parse_inheritance_list(t: Tokens) -> ParseResult<Vec<(PathNode, Vec<PathNode>)>> {
    alt((
        preceded(
            parse_symbol(Symbol::Colon),
            separated_list(
                parse_symbol(Symbol::Comma),
                tuple((
                    parse_path_node,
                    alt((
                        map(
                            tuple((
                                parse_symbol(Symbol::LeftBracket),
                                separated_nonempty_list(
                                    parse_symbol(Symbol::Comma),
                                    parse_path_node,
                                ),
                                parse_symbol(Symbol::RightBracket),
                            )),
                            |(_, path_list, _)| path_list,
                        ),
                        |s| Ok((s, vec![])),
                    )),
                )),
            ),
        ),
        |s| Ok((s, vec![])),
    ))(t)
}
