use super::*;

pub(super) fn transpile_literal_value_kind(node: &LiteralValueKind, c: &mut Context) {
  let _ = match node {
    LiteralValueKind::Bool(value) => write!(c, "{}", value),
    LiteralValueKind::Int(value) => write!(c, "{}", value),
    LiteralValueKind::Float(value) => write!(c, "{}", value),
    LiteralValueKind::String(value) => write!(c, r#""{}""#, value),
  };
}
