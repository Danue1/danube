mod analyze;
mod definition;
mod scan;

use crate::ErrorKind;
use danube_parser::*;
pub use definition::*;

type SymbolTableResult<T = ()> = Result<T, ErrorKind>;

pub fn create_type_symbol_table(node: &ProgramNode) -> SymbolTableResult<SymbolTable> {
  let mut type_symbol_table = SymbolTable::new("Root");
  type_symbol_table.scan_program_node(node)?;
  type_symbol_table.analyze()?;

  Ok(type_symbol_table)
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::collections::HashMap;
  use std::iter::FromIterator;

  fn compile(s: &str) -> SymbolTable {
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
    let source = "struct Foo(str);";
    assert_eq!(
      compile(source),
      SymbolTable {
        name: "Root".to_owned(),
        types: HashMap::from_iter(vec![
          ("bool".to_owned(), TypeSymbolKind::Primitive),
          ("int".to_owned(), TypeSymbolKind::Primitive),
          ("float".to_owned(), TypeSymbolKind::Primitive),
          ("str".to_owned(), TypeSymbolKind::Primitive),
          (
            "Entry".to_owned(),
            TypeSymbolKind::Module(ModuleSymbol {
              fields: HashMap::from_iter(vec![(
                "Foo".to_owned(),
                TypeSymbolKind::UnnamedStruct(UnnamedStructSymbol {
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
        ]),
        variables: HashMap::from_iter(vec![]),
      },
    );
  }
}
