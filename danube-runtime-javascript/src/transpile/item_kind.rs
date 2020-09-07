use super::*;

pub(super) fn transpile_item_kind(node: &ItemKind, c: &mut Context) {
  match node {
    ItemKind::Function(node) => transpile_function_node(&node, c),
    ItemKind::Constant(node) => transpile_constant_node(&node, c),
    _ => {}
  }
}
