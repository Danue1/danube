use super::*;

pub(super) fn transpile_constant_node(node: &ConstantNode, c: &mut Context) {
  let _ = write!(c, "const ");
  transpile_ident_node(&node.ident, c);
  let _ = write!(c, " = ");
  // TODO: add type
  transpile_expression_kind(&node.value, c);
  let _ = write!(c, ";");
}
