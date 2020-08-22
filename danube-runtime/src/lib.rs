use danube_parser::ModuleNode;

pub fn evaluate(m: ModuleNode) {
  println!("{:#?}", m);
}

#[cfg(test)]
mod tests {
  use super::*;
  use danube_parser::{lex, parse, Tokens};

  fn compile(s: &str) -> ModuleNode {
    let (_, token_list) = lex(s).unwrap();
    let (_, module) = parse(Tokens::new(&token_list)).unwrap();

    module
  }

  #[test]
  fn test() {
    let source = r#"fn main () {
      println("Hello, World!");
    }
    "#;
    let module = compile(source);
    evaluate(module);
  }
}
