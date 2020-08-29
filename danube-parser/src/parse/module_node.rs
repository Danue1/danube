use super::*;

pub(super) fn parse_module_node(s: Tokens) -> ParseResult<ModuleNode> {
  map(many0(parse_item_node), |item_list| ModuleNode {
    ident: None,
    item_list,
  })(s)
}
