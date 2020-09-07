use super::*;

pub(super) fn transpile_tuple_node(node: &TupleNode, c: &mut Context) {
  if let Some(field) = &node.field {
    let mut context = c.new_instance();
    transpile_expression_kind(field, &mut context);
    match context.runtime.as_str() {
      ident @ "print" => c.add_battery(ident.to_owned()),
      _ => (),
    }
    let _ = write!(c.runtime, "{}", context.runtime);
  }
  let _ = write!(c, "(");
  for argument in node.argument_list.iter() {
    transpile_tuple_argument_node(argument, c);
    let _ = write!(c, ",");
  }
  let _ = write!(c, ")");
}
