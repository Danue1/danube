use super::*;

pub(super) fn transpile_let_node(node: &LetNode, c: &mut Context) {
  match node.pattern {
    PatternKind::Placeholder => {
      if let Some(value) = &node.value {
        transpile_expression_kind(value, c);
      }
    }
    _ => {
      let _ = match node.immutablity {
        ImmutablityKind::Yes => write!(c, "const "),
        ImmutablityKind::Nope => write!(c, "let "),
      };
      transpile_pattern_kind(&node.pattern, c);
      if let Some(value) = &node.value {
        let _ = write!(c, "=");
        transpile_expression_kind(value, c);
      }
    }
  }
  let _ = write!(c, ";");
}
