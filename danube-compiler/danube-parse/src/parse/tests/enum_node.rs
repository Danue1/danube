use crate::Parse;
use danube_ast::{
    EnumNode, EnumVariantKind, EnumVariantNode, GenericNode, IdentNode, ImmutabilityKind, PathNode,
    TypeKind, TypeNode, DUMMY_NODE_ID,
};
use danube_lex::Lex;
use danube_token::{Symbol, Token};

#[test]
fn result() {
    let source = r#"
        Result<T, E> {
            Ok(T),
            Err(E),
        }
    "#;
    let tokens: Vec<Token> = Lex::new(source).filter_map(|token| token.ok()).collect();

    assert_eq!(
        Parse::new(tokens.as_slice()).parse_enum_node(),
        Ok(EnumNode {
            ident: IdentNode {
                id: DUMMY_NODE_ID,
                symbol: Symbol::intern("Result"),
            },
            generics: vec![
                GenericNode {
                    id: DUMMY_NODE_ID,
                    ident: IdentNode {
                        id: DUMMY_NODE_ID,
                        symbol: Symbol::intern("T"),
                    },
                    traits: vec![],
                    default: None,
                },
                GenericNode {
                    id: DUMMY_NODE_ID,
                    ident: IdentNode {
                        id: DUMMY_NODE_ID,
                        symbol: Symbol::intern("E"),
                    },
                    traits: vec![],
                    default: None,
                },
            ],
            variants: vec![
                EnumVariantNode {
                    id: DUMMY_NODE_ID,
                    ident: IdentNode {
                        id: DUMMY_NODE_ID,
                        symbol: Symbol::intern("Ok"),
                    },
                    kind: Some(EnumVariantKind::Unnamed(vec![TypeNode {
                        id: DUMMY_NODE_ID,
                        immutability: ImmutabilityKind::Yes,
                        kind: TypeKind::Path(PathNode {
                            segments: vec![IdentNode {
                                id: DUMMY_NODE_ID,
                                symbol: Symbol::intern("T"),
                            }],
                        }),
                    }])),
                },
                EnumVariantNode {
                    id: DUMMY_NODE_ID,
                    ident: IdentNode {
                        id: DUMMY_NODE_ID,
                        symbol: Symbol::intern("Err"),
                    },
                    kind: Some(EnumVariantKind::Unnamed(vec![TypeNode {
                        id: DUMMY_NODE_ID,
                        immutability: ImmutabilityKind::Yes,
                        kind: TypeKind::Path(PathNode {
                            segments: vec![IdentNode {
                                id: DUMMY_NODE_ID,
                                symbol: Symbol::intern("E"),
                            }],
                        }),
                    }])),
                },
            ],
        }),
    );
}
