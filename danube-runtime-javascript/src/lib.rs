mod transpile;

use danube_parse::*;
use transpile::*;

pub fn create_runtime(source: &str, config: &Config) -> String {
  let (_, token_list) = lex(source).unwrap();
  let (_, program_node) = parse(Tokens::new(&token_list)).unwrap();

  transpile(&program_node, &config)
}

#[test]
fn test() {
  let source = r#"
    fn main() {
      let message = "Hello, World!";
      print(message);
    }
  "#;
  let config = &Default::default();

  assert_eq!(
    create_runtime(source, config),
    r#"function main(){const message="Hello, World!";print(message,);}function print(message){console.log(message);}main();"#
  );
}
