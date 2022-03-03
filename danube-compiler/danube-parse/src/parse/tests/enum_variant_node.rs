use crate::{Context, Parse};
use danube_ast::{
    EnumVariantKind, EnumVariantNode, IdentNode, ImmutabilityKind, PathNode, TypeKind, TypeNode,
    DUMMY_NODE_ID,
};
use danube_lex::Lex;
use danube_token::{Symbol, Token};

#[test]
fn without_struct() {
    let source = "Foo";
    let tokens: Vec<Token> = Lex::new(source).filter_map(|token| token.ok()).collect();

    assert_eq!(
        EnumVariantNode::parse(&mut Context::new(tokens.as_slice())),
        Ok(EnumVariantNode {
            id: DUMMY_NODE_ID,
            ident: IdentNode {
                id: DUMMY_NODE_ID,
                symbol: Symbol::intern("Foo")
            },
            kind: None,
        }),
    );
}

#[test]
fn unnamed_without_fields() {
    let source = "Foo()";
    let tokens: Vec<Token> = Lex::new(source).filter_map(|token| token.ok()).collect();

    assert_eq!(
        EnumVariantNode::parse(&mut Context::new(tokens.as_slice())),
        Ok(EnumVariantNode {
            id: DUMMY_NODE_ID,
            ident: IdentNode {
                id: DUMMY_NODE_ID,
                symbol: Symbol::intern("Foo")
            },
            kind: Some(EnumVariantKind::Unnamed(vec![])),
        }),
    );
}

#[test]
fn unnamed_with_one_field() {
    let source = "Foo(Bar)";
    let tokens: Vec<Token> = Lex::new(source).filter_map(|token| token.ok()).collect();

    assert_eq!(
        EnumVariantNode::parse(&mut Context::new(tokens.as_slice())),
        Ok(EnumVariantNode {
            id: DUMMY_NODE_ID,
            ident: IdentNode {
                id: DUMMY_NODE_ID,
                symbol: Symbol::intern("Foo")
            },
            kind: Some(EnumVariantKind::Unnamed(vec![TypeNode {
                id: DUMMY_NODE_ID,
                immutability: ImmutabilityKind::Yes,
                kind: TypeKind::Path(PathNode {
                    segments: vec![IdentNode {
                        id: DUMMY_NODE_ID,
                        symbol: Symbol::intern("Bar"),
                    }],
                }),
            }])),
        }),
    );
}

#[test]
fn unnamed_with_two_field() {
    let source = "Foo(Bar, Baz)";
    let tokens: Vec<Token> = Lex::new(source).filter_map(|token| token.ok()).collect();

    assert_eq!(
        EnumVariantNode::parse(&mut Context::new(tokens.as_slice())),
        Ok(EnumVariantNode {
            id: DUMMY_NODE_ID,
            ident: IdentNode {
                id: DUMMY_NODE_ID,
                symbol: Symbol::intern("Foo")
            },
            kind: Some(EnumVariantKind::Unnamed(vec![
                TypeNode {
                    id: DUMMY_NODE_ID,
                    immutability: ImmutabilityKind::Yes,
                    kind: TypeKind::Path(PathNode {
                        segments: vec![IdentNode {
                            id: DUMMY_NODE_ID,
                            symbol: Symbol::intern("Bar"),
                        }],
                    }),
                },
                TypeNode {
                    id: DUMMY_NODE_ID,
                    immutability: ImmutabilityKind::Yes,
                    kind: TypeKind::Path(PathNode {
                        segments: vec![IdentNode {
                            id: DUMMY_NODE_ID,
                            symbol: Symbol::intern("Baz"),
                        }],
                    }),
                },
            ])),
        }),
    );
}

#[test]
fn named_with_one_field() {
    let source = "Foo { bar: Bar }";
    let tokens: Vec<Token> = Lex::new(source).filter_map(|token| token.ok()).collect();

    assert_eq!(
        EnumVariantNode::parse(&mut Context::new(tokens.as_slice())),
        Ok(EnumVariantNode {
            id: DUMMY_NODE_ID,
            ident: IdentNode {
                id: DUMMY_NODE_ID,
                symbol: Symbol::intern("Foo")
            },
            kind: Some(EnumVariantKind::Named(vec![(
                IdentNode {
                    id: DUMMY_NODE_ID,
                    symbol: Symbol::intern("bar"),
                },
                TypeNode {
                    id: DUMMY_NODE_ID,
                    immutability: ImmutabilityKind::Yes,
                    kind: TypeKind::Path(PathNode {
                        segments: vec![IdentNode {
                            id: DUMMY_NODE_ID,
                            symbol: Symbol::intern("Bar"),
                        }],
                    }),
                },
            )])),
        }),
    );
}

#[test]
fn named_with_two_field() {
    let source = "Foo { bar: Bar, baz: Baz }";
    let tokens: Vec<Token> = Lex::new(source).filter_map(|token| token.ok()).collect();

    assert_eq!(
        EnumVariantNode::parse(&mut Context::new(tokens.as_slice())),
        Ok(EnumVariantNode {
            id: DUMMY_NODE_ID,
            ident: IdentNode {
                id: DUMMY_NODE_ID,
                symbol: Symbol::intern("Foo")
            },
            kind: Some(EnumVariantKind::Named(vec![
                (
                    IdentNode {
                        id: DUMMY_NODE_ID,
                        symbol: Symbol::intern("bar"),
                    },
                    TypeNode {
                        id: DUMMY_NODE_ID,
                        immutability: ImmutabilityKind::Yes,
                        kind: TypeKind::Path(PathNode {
                            segments: vec![IdentNode {
                                id: DUMMY_NODE_ID,
                                symbol: Symbol::intern("Bar"),
                            }],
                        }),
                    },
                ),
                (
                    IdentNode {
                        id: DUMMY_NODE_ID,
                        symbol: Symbol::intern("baz"),
                    },
                    TypeNode {
                        id: DUMMY_NODE_ID,
                        immutability: ImmutabilityKind::Yes,
                        kind: TypeKind::Path(PathNode {
                            segments: vec![IdentNode {
                                id: DUMMY_NODE_ID,
                                symbol: Symbol::intern("Baz"),
                            }],
                        }),
                    },
                ),
            ])),
        }),
    );
}
