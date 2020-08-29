use super::*;

pub(super) fn parse_item_node(s: Tokens) -> ParseResult<ItemNode> {
  map(
    tuple((many0(parse_attribute_node), parse_item_kind)),
    |(attribute_list, kind)| ItemNode {
      attribute_list,
      kind,
    },
  )(s)
}
