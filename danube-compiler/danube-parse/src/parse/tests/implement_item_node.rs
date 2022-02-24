use crate::Parse;
use danube_ast::{
    BlockNode, ConstantNode, ExpressionKind, FunctionNode, IdentNode, ImmutabilityKind,
    ImplementItemKind, ImplementItemNode, PathNode, PatternKind, PatternNode, TypeAliasNode,
    TypeKind, TypeNode,
};
use danube_lex::Lex;
use danube_token::{Symbol, Token};

#[test]
fn type_without_type() {
    let source = "type Foo;";
    let tokens: Vec<Token> = Lex::new(source).filter_map(|token| token.ok()).collect();

    assert_eq!(
        Parse::new(tokens.as_slice()).parse_implement_item_node(),
        Ok(ImplementItemNode {
            attributes: vec![],
            kind: ImplementItemKind::Type(TypeAliasNode {
                ident: IdentNode {
                    symbol: Symbol::intern("Foo"),
                },
                ty: None,
            }),
        }),
    );
}

#[test]
fn type_with_type() {
    let source = "type Foo = Bar;";
    let tokens: Vec<Token> = Lex::new(source).filter_map(|token| token.ok()).collect();

    assert_eq!(
        Parse::new(tokens.as_slice()).parse_implement_item_node(),
        Ok(ImplementItemNode {
            attributes: vec![],
            kind: ImplementItemKind::Type(TypeAliasNode {
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
        }),
    );
}

#[test]
fn constant_without_value() {
    let source = "const FOO: Foo;";
    let tokens: Vec<Token> = Lex::new(source).filter_map(|token| token.ok()).collect();

    assert_eq!(
        Parse::new(tokens.as_slice()).parse_implement_item_node(),
        Ok(ImplementItemNode {
            attributes: vec![],
            kind: ImplementItemKind::Constant(ConstantNode {
                pattern: PatternNode {
                    kind: PatternKind::Path(PathNode {
                        segments: vec![IdentNode {
                            symbol: Symbol::intern("FOO"),
                        }],
                    }),
                },
                ty: TypeNode {
                    immutability: ImmutabilityKind::Yes,
                    kind: TypeKind::Path(PathNode {
                        segments: vec![IdentNode {
                            symbol: Symbol::intern("Foo"),
                        }],
                    }),
                },
                expression: None,
            }),
        }),
    );
}

#[test]
fn constant_with_value() {
    let source = "const FOO: Foo = foo;";
    let tokens: Vec<Token> = Lex::new(source).filter_map(|token| token.ok()).collect();

    assert_eq!(
        Parse::new(tokens.as_slice()).parse_implement_item_node(),
        Ok(ImplementItemNode {
            attributes: vec![],
            kind: ImplementItemKind::Constant(ConstantNode {
                pattern: PatternNode {
                    kind: PatternKind::Path(PathNode {
                        segments: vec![IdentNode {
                            symbol: Symbol::intern("FOO"),
                        }],
                    }),
                },
                ty: TypeNode {
                    immutability: ImmutabilityKind::Yes,
                    kind: TypeKind::Path(PathNode {
                        segments: vec![IdentNode {
                            symbol: Symbol::intern("Foo"),
                        }],
                    }),
                },
                expression: Some(ExpressionKind::Path(PathNode {
                    segments: vec![IdentNode {
                        symbol: Symbol::intern("foo"),
                    }],
                })),
            }),
        }),
    );
}

#[test]
fn function_without_body() {
    let source = "fn foo();";
    let tokens: Vec<Token> = Lex::new(source).filter_map(|token| token.ok()).collect();

    assert_eq!(
        Parse::new(tokens.as_slice()).parse_implement_item_node(),
        Ok(ImplementItemNode {
            attributes: vec![],
            kind: ImplementItemKind::Function(FunctionNode {
                ident: IdentNode {
                    symbol: Symbol::intern("foo"),
                },
                generics: vec![],
                self_type: None,
                parameters: vec![],
                return_type: None,
                block: None,
            }),
        }),
    );
}

#[test]
fn function_with_body() {
    let source = "fn foo() {}";
    let tokens: Vec<Token> = Lex::new(source).filter_map(|token| token.ok()).collect();

    assert_eq!(
        Parse::new(tokens.as_slice()).parse_implement_item_node(),
        Ok(ImplementItemNode {
            attributes: vec![],
            kind: ImplementItemKind::Function(FunctionNode {
                ident: IdentNode {
                    symbol: Symbol::intern("foo"),
                },
                generics: vec![],
                self_type: None,
                parameters: vec![],
                return_type: None,
                block: Some(BlockNode { statements: vec![] }),
            }),
        }),
    );
}
