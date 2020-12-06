use super::*;

pub fn parse_module_node(t: Tokens) -> ParseResult<ModuleNode> {
    map(many0(parse_attributed(parse_item_kind)), |item_list| {
        ModuleNode { item_list }
    })(t)
}
