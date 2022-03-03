use crate::{Context, Parse};
use danube_ast::{
    BlockNode, FunctionNode, FunctionParameterNode, IdentNode, ImmutabilityKind, PathNode,
    TypeKind, TypeNode, DUMMY_NODE_ID,
};
use danube_lex::Lex;
use danube_token::{Symbol, Token};

#[test]
fn without_block() {
    let source = "foo();";
    let tokens: Vec<Token> = Lex::new(source).filter_map(|token| token.ok()).collect();

    assert_eq!(
        FunctionNode::parse(&mut Context::new(tokens.as_slice())),
        Ok(FunctionNode {
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
    );
}

#[test]
fn with_block() {
    let source = "foo() { }";
    let tokens: Vec<Token> = Lex::new(source).filter_map(|token| token.ok()).collect();

    assert_eq!(
        FunctionNode::parse(&mut Context::new(tokens.as_slice())),
        Ok(FunctionNode {
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
    );
}

#[test]
fn with_return_type() {
    let source = "foo() -> bar;";
    let tokens: Vec<Token> = Lex::new(source).filter_map(|token| token.ok()).collect();

    assert_eq!(
        FunctionNode::parse(&mut Context::new(tokens.as_slice())),
        Ok(FunctionNode {
            ident: IdentNode {
                id: DUMMY_NODE_ID,
                symbol: Symbol::intern("foo"),
            },
            generics: vec![],
            self_type: None,
            parameters: vec![],
            return_type: Some(TypeNode {
                id: DUMMY_NODE_ID,
                immutability: ImmutabilityKind::Yes,
                kind: TypeKind::Path(PathNode {
                    segments: vec![IdentNode {
                        id: DUMMY_NODE_ID,
                        symbol: Symbol::intern("bar"),
                    }],
                }),
            }),
            block: None,
        }),
    );
}

#[test]
fn immutable_self() {
    let source = "foo(self);";
    let tokens: Vec<Token> = Lex::new(source).filter_map(|token| token.ok()).collect();

    assert_eq!(
        FunctionNode::parse(&mut Context::new(tokens.as_slice())),
        Ok(FunctionNode {
            ident: IdentNode {
                id: DUMMY_NODE_ID,
                symbol: Symbol::intern("foo"),
            },
            generics: vec![],
            self_type: Some(ImmutabilityKind::Yes),
            parameters: vec![],
            return_type: None,
            block: None,
        }),
    );
}

#[test]
fn mutable_self() {
    let source = "foo(mut self);";
    let tokens: Vec<Token> = Lex::new(source).filter_map(|token| token.ok()).collect();

    assert_eq!(
        FunctionNode::parse(&mut Context::new(tokens.as_slice())),
        Ok(FunctionNode {
            ident: IdentNode {
                id: DUMMY_NODE_ID,
                symbol: Symbol::intern("foo"),
            },
            generics: vec![],
            self_type: Some(ImmutabilityKind::Nope),
            parameters: vec![],
            return_type: None,
            block: None,
        }),
    );
}

#[test]
fn one_parameter() {
    let source = "foo(bar: Bar);";
    let tokens: Vec<Token> = Lex::new(source).filter_map(|token| token.ok()).collect();

    assert_eq!(
        FunctionNode::parse(&mut Context::new(tokens.as_slice())),
        Ok(FunctionNode {
            ident: IdentNode {
                id: DUMMY_NODE_ID,
                symbol: Symbol::intern("foo"),
            },
            generics: vec![],
            self_type: None,
            parameters: vec![FunctionParameterNode {
                id: DUMMY_NODE_ID,
                argument_label: IdentNode {
                    id: DUMMY_NODE_ID,
                    symbol: Symbol::intern("bar"),
                },
                parameter_label: None,
                ty: TypeNode {
                    id: DUMMY_NODE_ID,
                    immutability: ImmutabilityKind::Yes,
                    kind: TypeKind::Path(PathNode {
                        segments: vec![IdentNode {
                            id: DUMMY_NODE_ID,
                            symbol: Symbol::intern("Bar"),
                        }],
                    }),
                }
            }],
            return_type: None,
            block: None,
        }),
    );
}

#[test]
fn two_parameters() {
    let source = "foo(bar: Bar, baz: Baz);";
    let tokens: Vec<Token> = Lex::new(source).filter_map(|token| token.ok()).collect();

    assert_eq!(
        FunctionNode::parse(&mut Context::new(tokens.as_slice())),
        Ok(FunctionNode {
            ident: IdentNode {
                id: DUMMY_NODE_ID,
                symbol: Symbol::intern("foo"),
            },
            generics: vec![],
            self_type: None,
            parameters: vec![
                FunctionParameterNode {
                    id: DUMMY_NODE_ID,
                    argument_label: IdentNode {
                        id: DUMMY_NODE_ID,
                        symbol: Symbol::intern("bar"),
                    },
                    parameter_label: None,
                    ty: TypeNode {
                        id: DUMMY_NODE_ID,
                        immutability: ImmutabilityKind::Yes,
                        kind: TypeKind::Path(PathNode {
                            segments: vec![IdentNode {
                                id: DUMMY_NODE_ID,
                                symbol: Symbol::intern("Bar"),
                            }],
                        }),
                    }
                },
                FunctionParameterNode {
                    id: DUMMY_NODE_ID,
                    argument_label: IdentNode {
                        id: DUMMY_NODE_ID,
                        symbol: Symbol::intern("baz"),
                    },
                    parameter_label: None,
                    ty: TypeNode {
                        id: DUMMY_NODE_ID,
                        immutability: ImmutabilityKind::Yes,
                        kind: TypeKind::Path(PathNode {
                            segments: vec![IdentNode {
                                id: DUMMY_NODE_ID,
                                symbol: Symbol::intern("Baz"),
                            }],
                        }),
                    }
                },
            ],
            return_type: None,
            block: None,
        }),
    );
}
