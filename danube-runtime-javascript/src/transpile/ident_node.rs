use super::*;

pub(super) fn transpile_ident_node(node: &IdentNode, c: &mut Context) {
  let _ = write!(c, "{}", &node.raw);
}
