use super::*;

pub(super) fn transpile_function_argument_node(node: &FunctionArgumentNode, c: &mut Context) {
  transpile_ident_node(&node.ident, c);
  // TODO: add type
}
