use super::*;

pub(super) fn transpile_pattern_kind(node: &PatternKind, c: &mut Context) {
  match node {
    PatternKind::Placeholder => transpile_placeholder(c),
    PatternKind::Literal(node) => transpile_literal_value_kind(&node, c),
    PatternKind::Path(node) => transpile_path_node(&node, c),
    _ => {}
  };
}

fn transpile_placeholder(_c: &mut Context) {
  //
}
