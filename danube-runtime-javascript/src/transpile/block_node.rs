use super::*;

pub(super) fn transpile_block_node(node: &BlockNode, c: &mut Context) {
  for statement in node.statement_list.iter() {
    transpile_statement_kind(&statement, c);
  }
}
