use crate::{Context, Parse};
use danube_ast::{
    BlockNode, ConstantNode, ExpressionKind, ExpressionNode, FunctionNode, IdentNode,
    ImmutabilityKind, ImplementItemKind, ImplementItemNode, PathNode, PatternKind, PatternNode,
    TypeAliasNode, TypeKind, TypeNode, DUMMY_NODE_ID,
};
use danube_lex::Lex;
use danube_token::{Symbol, Token};

#[test]
fn type_without_type() {
    let source = "type Foo;";
    let tokens: Vec<Token> = Lex::new(source).filter_map(|token| token.ok()).collect();

    assert_eq!(
        ImplementItemNode::parse(&mut Context::new(tokens.as_slice())),
        Ok(ImplementItemNode {
            id: DUMMY_NODE_ID,
            attributes: vec![],
            kind: ImplementItemKind::Type(TypeAliasNode {
                ident: IdentNode {
                    id: DUMMY_NODE_ID,
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
        ImplementItemNode::parse(&mut Context::new(tokens.as_slice())),
        Ok(ImplementItemNode {
            id: DUMMY_NODE_ID,
            attributes: vec![],
            kind: ImplementItemKind::Type(TypeAliasNode {
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
        }),
    );
}

#[test]
fn constant_without_value() {
    let source = "const FOO: Foo;";
    let tokens: Vec<Token> = Lex::new(source).filter_map(|token| token.ok()).collect();

    assert_eq!(
        ImplementItemNode::parse(&mut Context::new(tokens.as_slice())),
        Ok(ImplementItemNode {
            id: DUMMY_NODE_ID,
            attributes: vec![],
            kind: ImplementItemKind::Constant(ConstantNode {
                pattern: PatternNode {
                    id: DUMMY_NODE_ID,
                    kind: PatternKind::Path(PathNode {
                        segments: vec![IdentNode {
                            id: DUMMY_NODE_ID,
                            symbol: Symbol::intern("FOO"),
                        }],
                    }),
                },
                ty: TypeNode {
                    id: DUMMY_NODE_ID,
                    immutability: ImmutabilityKind::Yes,
                    kind: TypeKind::Path(PathNode {
                        segments: vec![IdentNode {
                            id: DUMMY_NODE_ID,
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
        ImplementItemNode::parse(&mut Context::new(tokens.as_slice())),
        Ok(ImplementItemNode {
            id: DUMMY_NODE_ID,
            attributes: vec![],
            kind: ImplementItemKind::Constant(ConstantNode {
                pattern: PatternNode {
                    id: DUMMY_NODE_ID,
                    kind: PatternKind::Path(PathNode {
                        segments: vec![IdentNode {
                            id: DUMMY_NODE_ID,
                            symbol: Symbol::intern("FOO"),
                        }],
                    }),
                },
                ty: TypeNode {
                    id: DUMMY_NODE_ID,
                    immutability: ImmutabilityKind::Yes,
                    kind: TypeKind::Path(PathNode {
                        segments: vec![IdentNode {
                            id: DUMMY_NODE_ID,
                            symbol: Symbol::intern("Foo"),
                        }],
                    }),
                },
                expression: Some(ExpressionNode {
                    id: DUMMY_NODE_ID,
                    kind: ExpressionKind::Path(PathNode {
                        segments: vec![IdentNode {
                            id: DUMMY_NODE_ID,
                            symbol: Symbol::intern("foo"),
                        }],
                    })
                }),
            }),
        }),
    );
}

#[test]
fn function_without_body() {
    let source = "fn foo();";
    let tokens: Vec<Token> = Lex::new(source).filter_map(|token| token.ok()).collect();

    assert_eq!(
        ImplementItemNode::parse(&mut Context::new(tokens.as_slice())),
        Ok(ImplementItemNode {
            id: DUMMY_NODE_ID,
            attributes: vec![],
            kind: ImplementItemKind::Function(FunctionNode {
                ident: IdentNode {
                    id: DUMMY_NODE_ID,
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
        ImplementItemNode::parse(&mut Context::new(tokens.as_slice())),
        Ok(ImplementItemNode {
            id: DUMMY_NODE_ID,
            attributes: vec![],
            kind: ImplementItemKind::Function(FunctionNode {
                ident: IdentNode {
                    id: DUMMY_NODE_ID,
                    symbol: Symbol::intern("foo"),
                },
                generics: vec![],
                self_type: None,
                parameters: vec![],
                return_type: None,
                block: Some(BlockNode {
                    id: DUMMY_NODE_ID,
                    statements: vec![]
                }),
            }),
        }),
    );
}
