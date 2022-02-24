use crate::Parse;
use danube_ast::{
    EnumVariantKind, EnumVariantNode, IdentNode, ImmutabilityKind, PathNode, TypeKind, TypeNode,
};
use danube_lex::Lex;
use danube_token::{Symbol, Token};

#[test]
fn without_struct() {
    let source = "Foo";
    let tokens: Vec<Token> = Lex::new(source).filter_map(|token| token.ok()).collect();

    assert_eq!(
        Parse::new(tokens.as_slice()).parse_enum_variant_node(),
        Ok(EnumVariantNode {
            ident: IdentNode {
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
        Parse::new(tokens.as_slice()).parse_enum_variant_node(),
        Ok(EnumVariantNode {
            ident: IdentNode {
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
        Parse::new(tokens.as_slice()).parse_enum_variant_node(),
        Ok(EnumVariantNode {
            ident: IdentNode {
                symbol: Symbol::intern("Foo")
            },
            kind: Some(EnumVariantKind::Unnamed(vec![TypeNode {
                immutability: ImmutabilityKind::Yes,
                kind: TypeKind::Path(PathNode {
                    segments: vec![IdentNode {
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
        Parse::new(tokens.as_slice()).parse_enum_variant_node(),
        Ok(EnumVariantNode {
            ident: IdentNode {
                symbol: Symbol::intern("Foo")
            },
            kind: Some(EnumVariantKind::Unnamed(vec![
                TypeNode {
                    immutability: ImmutabilityKind::Yes,
                    kind: TypeKind::Path(PathNode {
                        segments: vec![IdentNode {
                            symbol: Symbol::intern("Bar"),
                        }],
                    }),
                },
                TypeNode {
                    immutability: ImmutabilityKind::Yes,
                    kind: TypeKind::Path(PathNode {
                        segments: vec![IdentNode {
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
        Parse::new(tokens.as_slice()).parse_enum_variant_node(),
        Ok(EnumVariantNode {
            ident: IdentNode {
                symbol: Symbol::intern("Foo")
            },
            kind: Some(EnumVariantKind::Named(vec![(
                IdentNode {
                    symbol: Symbol::intern("bar"),
                },
                TypeNode {
                    immutability: ImmutabilityKind::Yes,
                    kind: TypeKind::Path(PathNode {
                        segments: vec![IdentNode {
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
        Parse::new(tokens.as_slice()).parse_enum_variant_node(),
        Ok(EnumVariantNode {
            ident: IdentNode {
                symbol: Symbol::intern("Foo")
            },
            kind: Some(EnumVariantKind::Named(vec![
                (
                    IdentNode {
                        symbol: Symbol::intern("bar"),
                    },
                    TypeNode {
                        immutability: ImmutabilityKind::Yes,
                        kind: TypeKind::Path(PathNode {
                            segments: vec![IdentNode {
                                symbol: Symbol::intern("Bar"),
                            }],
                        }),
                    },
                ),
                (
                    IdentNode {
                        symbol: Symbol::intern("baz"),
                    },
                    TypeNode {
                        immutability: ImmutabilityKind::Yes,
                        kind: TypeKind::Path(PathNode {
                            segments: vec![IdentNode {
                                symbol: Symbol::intern("Baz"),
                            }],
                        }),
                    },
                ),
            ])),
        }),
    );
}
