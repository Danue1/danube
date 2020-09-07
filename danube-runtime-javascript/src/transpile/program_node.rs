use super::*;

pub(super) fn transpile_program_node(node: &ProgramNode, c: &mut Context) {
  for item in node.item_list.iter() {
    transpile_item_node(item, c);
  }
  transpile_batteries(c);
  let _ = write!(c, "main();");
}

fn transpile_batteries(c: &mut Context) {
  let Context {
    runtime,
    batteries,
    config,
  } = c;
  for battery in batteries.iter() {
    match battery.as_str() {
      "print" => transpile_print(runtime, config),
      _ => {}
    }
  }
}

fn transpile_print(c: &mut String, config: &Config) {
  if config.language.is_javascript() {
    let _ = write!(c, "function print(message){{console.log(message);}}");
  } else {
    let _ = write!(c, "function print(message: any){{console.log(message);}}");
  }
}
