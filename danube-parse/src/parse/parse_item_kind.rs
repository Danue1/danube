use super::*;

pub fn parse_item_kind(t: Tokens) -> ParseResult<ItemKind> {
    map(parse_function_node, ItemKind::Function)(t)
}
