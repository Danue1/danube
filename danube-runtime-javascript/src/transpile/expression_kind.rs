use super::*;

pub(super) fn transpile_expression_kind(node: &ExpressionKind, c: &mut Context) {
  match node {
    ExpressionKind::Path(node) => transpile_path_node(&node, c),
    ExpressionKind::Break => transpile_break(c),
    ExpressionKind::Continue => transpile_continue(c),
    ExpressionKind::Return(node) => transpile_return_node(node, c),
    ExpressionKind::Literal(node) => transpile_literal_value_kind(&node, c),
    ExpressionKind::Array(node) => transpile_array(&node, c),
    ExpressionKind::Tuple(node) => transpile_tuple_node(&node, c),
    ExpressionKind::Await(node) => transpile_await(&node, c),
    ExpressionKind::Try(node) => transpile_try(&node, c),
    ExpressionKind::Block(node) => transpile_block_node(&node, c),
    _ => {}
  }
}

fn transpile_break(c: &mut Context) {
  let _ = write!(c, "break;");
}

fn transpile_continue(c: &mut Context) {
  let _ = write!(c, "continue;");
}

fn transpile_array(node_list: &[ExpressionKind], c: &mut Context) {
  for node in node_list.iter() {
    transpile_expression_kind(node, c);
  }
}

fn transpile_await(node: &ExpressionKind, c: &mut Context) {
  let _ = write!(c, "(await ");
  transpile_expression_kind(node, c);
  let _ = write!(c, ");");
}

fn transpile_try(_node: &ExpressionKind, _c: &mut Context) {
  //
}
