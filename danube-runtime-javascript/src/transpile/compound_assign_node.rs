use super::*;

pub(super) fn transpile_compound_assign_node(node: &CompoundAssignNode, c: &mut Context) {
  transpile_expression_kind(&node.lhs, c);
  let _ = write!(c, " ");
  transpile_compound_assign_kind(&node.kind, c);
  let _ = write!(c, " ");
  transpile_expression_kind(&node.rhs, c);
}
