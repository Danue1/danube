mod analyze;
mod definition;
mod scan;

use crate::ErrorKind;
use danube_parser::*;
pub use definition::*;

type TypeSymbolResult<T = ()> = Result<T, ErrorKind>;

pub fn create_type_symbol_table(node: &ProgramNode) -> TypeSymbolResult<TypeSymbolTable> {
  let mut type_symbol_table = TypeSymbolTable::new("Root");
  type_symbol_table.scan_program_node(node)?;
  type_symbol_table.analyze()?;

  Ok(type_symbol_table)
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::collections::HashMap;
  use std::iter::FromIterator;

  fn compile(s: &str) -> TypeSymbolTable {
    let (_, token_list) = lex(s).unwrap();
    let (_, program_node) = parse(Tokens::new(&token_list)).unwrap();
    match create_type_symbol_table(&program_node) {
      Ok(node) => node,
      Err(error) => {
        dbg!(error);
        panic!()
      }
    }
  }

  #[test]
  fn unnamed_struct() {
    "
      type Bool = bool;
      type Int = int;
      type Float = float;
      type String = str;
    ";
    let source = "struct Foo(str);";
    assert_eq!(
      compile(source),
      TypeSymbolTable {
        name: "Root".to_owned(),
        symbol_tables: HashMap::from_iter(vec![
          ("bool".to_owned(), TypeSymbolKind::Primitive),
          ("int".to_owned(), TypeSymbolKind::Primitive),
          ("float".to_owned(), TypeSymbolKind::Primitive),
          ("str".to_owned(), TypeSymbolKind::Primitive),
          (
            "Entry".to_owned(),
            TypeSymbolKind::Module(Module {
              fields: HashMap::from_iter(vec![(
                "Foo".to_owned(),
                TypeSymbolKind::UnnamedStruct(UnnamedStruct {
                  fields: vec![TypeKind::Path(
                    ImmutablityKind::Yes,
                    PathNode {
                      ident_list: vec![IdentNode {
                        raw: "str".to_owned()
                      }]
                    }
                  )]
                })
              )])
            })
          )
        ])
      }
    );
  }
}
