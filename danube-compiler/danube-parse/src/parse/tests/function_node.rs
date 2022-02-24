use crate::Parse;
use danube_ast::{
    BlockNode, FunctionNode, FunctionParameterNode, IdentNode, ImmutablityKind, PathNode, TypeKind,
    TypeNode,
};
use danube_lex::Lex;
use danube_token::{Symbol, Token};

#[test]
fn without_block() {
    let source = "foo();";
    let tokens: Vec<Token> = Lex::new(source).filter_map(|token| token.ok()).collect();

    assert_eq!(
        Parse::new(tokens.as_slice()).parse_function_node(),
        Ok(FunctionNode {
            ident: IdentNode {
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
        Parse::new(tokens.as_slice()).parse_function_node(),
        Ok(FunctionNode {
            ident: IdentNode {
                symbol: Symbol::intern("foo"),
            },
            generics: vec![],
            self_type: None,
            parameters: vec![],
            return_type: None,
            block: Some(BlockNode { statements: vec![] }),
        }),
    );
}

#[test]
fn with_return_type() {
    let source = "foo() -> bar;";
    let tokens: Vec<Token> = Lex::new(source).filter_map(|token| token.ok()).collect();

    assert_eq!(
        Parse::new(tokens.as_slice()).parse_function_node(),
        Ok(FunctionNode {
            ident: IdentNode {
                symbol: Symbol::intern("foo"),
            },
            generics: vec![],
            self_type: None,
            parameters: vec![],
            return_type: Some(TypeNode {
                immutablity: ImmutablityKind::Yes,
                kind: TypeKind::Path(PathNode {
                    segments: vec![IdentNode {
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
        Parse::new(tokens.as_slice()).parse_function_node(),
        Ok(FunctionNode {
            ident: IdentNode {
                symbol: Symbol::intern("foo"),
            },
            generics: vec![],
            self_type: Some(ImmutablityKind::Yes),
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
        Parse::new(tokens.as_slice()).parse_function_node(),
        Ok(FunctionNode {
            ident: IdentNode {
                symbol: Symbol::intern("foo"),
            },
            generics: vec![],
            self_type: Some(ImmutablityKind::Nope),
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
        Parse::new(tokens.as_slice()).parse_function_node(),
        Ok(FunctionNode {
            ident: IdentNode {
                symbol: Symbol::intern("foo"),
            },
            generics: vec![],
            self_type: None,
            parameters: vec![FunctionParameterNode {
                argument_label: IdentNode {
                    symbol: Symbol::intern("bar"),
                },
                parameter_label: None,
                ty: TypeNode {
                    immutablity: ImmutablityKind::Yes,
                    kind: TypeKind::Path(PathNode {
                        segments: vec![IdentNode {
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
        Parse::new(tokens.as_slice()).parse_function_node(),
        Ok(FunctionNode {
            ident: IdentNode {
                symbol: Symbol::intern("foo"),
            },
            generics: vec![],
            self_type: None,
            parameters: vec![
                FunctionParameterNode {
                    argument_label: IdentNode {
                        symbol: Symbol::intern("bar"),
                    },
                    parameter_label: None,
                    ty: TypeNode {
                        immutablity: ImmutablityKind::Yes,
                        kind: TypeKind::Path(PathNode {
                            segments: vec![IdentNode {
                                symbol: Symbol::intern("Bar"),
                            }],
                        }),
                    }
                },
                FunctionParameterNode {
                    argument_label: IdentNode {
                        symbol: Symbol::intern("baz"),
                    },
                    parameter_label: None,
                    ty: TypeNode {
                        immutablity: ImmutablityKind::Yes,
                        kind: TypeKind::Path(PathNode {
                            segments: vec![IdentNode {
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
