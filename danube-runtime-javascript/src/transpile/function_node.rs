use super::*;

pub(super) fn transpile_function_node(node: &FunctionNode, c: &mut Context) {
  if node.is_async {
    let _ = write!(c, "async ");
  }
  let _ = write!(c, "function ");
  transpile_ident_node(&node.ident, c);
  let _ = write!(c, "(");
  for argument in node.argument_list.iter() {
    transpile_function_argument_node(argument, c);
    let _ = write!(c, ", ");
  }
  let _ = write!(c, "){{");
  transpile_block_node(&node.block, c);
  let _ = write!(c, "}}");
}
