use super::*;

pub(super) fn transpile_statement_kind(node: &StatementKind, c: &mut Context) {
  match node {
    StatementKind::Item(node) => transpile_item_node(&node, c),
    StatementKind::CompoundAssign(node) => transpile_compound_assign_node(&node, c),
    StatementKind::Let(node) => transpile_let_node(&node, c),
    StatementKind::ExpressionKind(node) => transpile_expression_kind(&node, c),
    StatementKind::Semicolon => transpile_semicolon(c),
  }
}

fn transpile_semicolon(c: &mut Context) {
  let _ = write!(c, ";");
}
