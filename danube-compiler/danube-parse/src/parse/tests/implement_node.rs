use crate::{Context, Parse};
use danube_ast::{GenericNode, IdentNode, ImplementNode, PathNode, DUMMY_NODE_ID};
use danube_lex::Lex;
use danube_token::{Symbol, Token};

#[test]
fn with_nothing() {
    let source = "Foo {}";
    let tokens: Vec<Token> = Lex::new(source).filter_map(|token| token.ok()).collect();

    assert_eq!(
        ImplementNode::parse(&mut Context::new(tokens.as_slice())),
        Ok(ImplementNode {
            generics: vec![],
            trait_ident: None,
            target: PathNode {
                segments: vec![IdentNode {
                    id: DUMMY_NODE_ID,
                    symbol: Symbol::intern("Foo"),
                }],
            },
            target_generics: vec![],
            items: vec![],
        }),
    );
}

#[test]
fn with_generics() {
    let source = "<T>Foo {}";
    let tokens: Vec<Token> = Lex::new(source).filter_map(|token| token.ok()).collect();

    assert_eq!(
        ImplementNode::parse(&mut Context::new(tokens.as_slice())),
        Ok(ImplementNode {
            generics: vec![GenericNode {
                id: DUMMY_NODE_ID,
                ident: IdentNode {
                    id: DUMMY_NODE_ID,
                    symbol: Symbol::intern("T"),
                },
                traits: vec![],
                default: None,
            }],
            trait_ident: None,
            target: PathNode {
                segments: vec![IdentNode {
                    id: DUMMY_NODE_ID,
                    symbol: Symbol::intern("Foo"),
                }],
            },
            target_generics: vec![],
            items: vec![],
        }),
    );
}

#[test]
fn target_generics_with_generics() {
    let source = "<T>Foo<T> {}";
    let tokens: Vec<Token> = Lex::new(source).filter_map(|token| token.ok()).collect();

    assert_eq!(
        ImplementNode::parse(&mut Context::new(tokens.as_slice())),
        Ok(ImplementNode {
            generics: vec![GenericNode {
                id: DUMMY_NODE_ID,
                ident: IdentNode {
                    id: DUMMY_NODE_ID,
                    symbol: Symbol::intern("T"),
                },
                traits: vec![],
                default: None,
            }],
            trait_ident: None,
            target: PathNode {
                segments: vec![IdentNode {
                    id: DUMMY_NODE_ID,
                    symbol: Symbol::intern("Foo"),
                }],
            },
            target_generics: vec![GenericNode {
                id: DUMMY_NODE_ID,
                ident: IdentNode {
                    id: DUMMY_NODE_ID,
                    symbol: Symbol::intern("T"),
                },
                traits: vec![],
                default: None,
            }],
            items: vec![],
        }),
    );
}

#[test]
fn for_with_generics() {
    let source = "Foo for Bar {}";
    let tokens: Vec<Token> = Lex::new(source).filter_map(|token| token.ok()).collect();

    assert_eq!(
        ImplementNode::parse(&mut Context::new(tokens.as_slice())),
        Ok(ImplementNode {
            generics: vec![],
            trait_ident: Some(PathNode {
                segments: vec![IdentNode {
                    id: DUMMY_NODE_ID,
                    symbol: Symbol::intern("Foo"),
                }],
            }),
            target: PathNode {
                segments: vec![IdentNode {
                    id: DUMMY_NODE_ID,
                    symbol: Symbol::intern("Bar"),
                }],
            },
            target_generics: vec![],
            items: vec![],
        }),
    );
}

#[test]
fn for_generics_with_generics() {
    let source = "Foo for Bar<T> {}";
    let tokens: Vec<Token> = Lex::new(source).filter_map(|token| token.ok()).collect();

    assert_eq!(
        ImplementNode::parse(&mut Context::new(tokens.as_slice())),
        Ok(ImplementNode {
            generics: vec![],
            trait_ident: Some(PathNode {
                segments: vec![IdentNode {
                    id: DUMMY_NODE_ID,
                    symbol: Symbol::intern("Foo"),
                }],
            }),
            target: PathNode {
                segments: vec![IdentNode {
                    id: DUMMY_NODE_ID,
                    symbol: Symbol::intern("Bar"),
                }],
            },
            target_generics: vec![GenericNode {
                id: DUMMY_NODE_ID,
                ident: IdentNode {
                    id: DUMMY_NODE_ID,
                    symbol: Symbol::intern("T"),
                },
                traits: vec![],
                default: None,
            }],
            items: vec![],
        }),
    );
}
