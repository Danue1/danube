use danube_ast::{
  IdentNode, ImmutabilityKind, PathNode, TypeAliasNode, TypeKind, TypeNode, DUMMY_NODE_ID,
};
use danube_token::Symbol;

assert_node! {
  #[test]
  fn without_type() -> TypeAliasNode {
    let source = "Foo;";

    assert_eq!(
      source,
      Ok(TypeAliasNode {
        id: DUMMY_NODE_ID,
        ident: IdentNode {
          id: DUMMY_NODE_ID,
          symbol: Symbol::intern("Foo"),
        },
        ty: None,
      }),
    );
  }

  #[test]
  fn with_type() -> TypeAliasNode {
    let source = "Foo = Bar;";

    assert_eq!(
      source,
      Ok(TypeAliasNode {
        id: DUMMY_NODE_ID,
        ident: IdentNode {
          id: DUMMY_NODE_ID,
          symbol: Symbol::intern("Foo"),
        },
        ty: Some(TypeNode {
          id: DUMMY_NODE_ID,
          immutability: ImmutabilityKind::Yes,
          kind: TypeKind::Path(PathNode {
            segments: vec![IdentNode {
              id: DUMMY_NODE_ID,
              symbol: Symbol::intern("Bar"),
            }],
          }),
        }),
      }),
    );
  }
}
