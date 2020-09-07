use super::*;

pub(super) fn transpile_path_node(node: &PathNode, c: &mut Context) {
  for ident in node.ident_list.iter() {
    transpile_ident_node(ident, c);
  }
}
