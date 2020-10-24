use super::*;

pub(super) fn parse_module_node(t: Tokens) -> ParseResult<ModuleNode> {
    map(
        tuple((
            parse_keyword(Keyword::Module),
            parse_ident_node,
            parse_optional_item_list,
        )),
        |(_, ident, item_list)| ModuleNode { ident, item_list },
    )(t)
}

fn parse_optional_item_list(t: Tokens) -> ParseResult<Option<Vec<Attributed<ItemKind>>>> {
    alt((
        map(parse_symbol(Symbol::Semicolon), |_| None),
        map(parse_item_list, Some),
    ))(t)
}

fn parse_item_list(t: Tokens) -> ParseResult<Vec<Attributed<ItemKind>>> {
    map(
        tuple((
            parse_symbol(Symbol::LeftBrace),
            many1(parse_attributed(parse_item_kind)),
            parse_symbol(Symbol::RightBrace),
        )),
        |(_, item_list, _)| item_list,
    )(t)
}
