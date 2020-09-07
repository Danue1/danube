use super::*;

pub(super) fn transpile_item_node(node: &ItemNode, c: &mut Context) {
  transpile_item_kind(&node.kind, c);
}
