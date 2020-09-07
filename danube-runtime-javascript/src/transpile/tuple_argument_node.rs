use super::*;

pub(super) fn transpile_tuple_argument_node(node: &TupleArgumentNode, c: &mut Context) {
  // TODO: add named parameter
  transpile_expression_kind(&node.value, c);
}
