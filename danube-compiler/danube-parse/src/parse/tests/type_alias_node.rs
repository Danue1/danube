use crate::Parse;
use danube_ast::{IdentNode, ImmutabilityKind, PathNode, TypeAliasNode, TypeKind, TypeNode};
use danube_lex::Lex;
use danube_token::{Symbol, Token};

#[test]
fn without_type() {
  let source = "Foo;";
  let tokens: Vec<Token> = Lex::new(source).filter_map(|token| token.ok()).collect();

  assert_eq!(
    Parse::new(tokens.as_slice()).parse_type_alias_node(),
    Ok(TypeAliasNode {
      ident: IdentNode {
        symbol: Symbol::intern("Foo"),
      },
      ty: None,
    }),
  );
}

#[test]
fn with_type() {
  let source = "Foo = Bar;";
  let tokens: Vec<Token> = Lex::new(source).filter_map(|token| token.ok()).collect();

  assert_eq!(
    Parse::new(tokens.as_slice()).parse_type_alias_node(),
    Ok(TypeAliasNode {
      ident: IdentNode {
        symbol: Symbol::intern("Foo"),
      },
      ty: Some(TypeNode {
        immutability: ImmutabilityKind::Yes,
        kind: TypeKind::Path(PathNode {
          segments: vec![IdentNode {
            symbol: Symbol::intern("Bar"),
          }],
        }),
      }),
    }),
  );
}