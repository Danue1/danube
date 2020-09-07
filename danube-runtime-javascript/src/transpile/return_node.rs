use super::*;

pub(super) fn transpile_return_node(node: &ReturnNode, c: &mut Context) {
  let _ = write!(c, "return");
  if let Some(value) = &node.value {
    let _ = write!(c, " ");
    transpile_expression_kind(&value, c);
  }
  let _ = write!(c, ";");
}
