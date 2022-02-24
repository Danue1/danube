use crate::Parse;
use danube_ast::{
    AssignKind, AssignNode, BinaryExpressionNode, BinaryOperatorKind, ExpressionKind, Id,
    IdentNode, ImmutabilityKind, LetNode, PathNode, PatternKind, PatternNode, StatementId,
    StatementKind, StatementNode, TypeKind, TypeNode,
};
use danube_lex::Lex;
use danube_token::{Symbol, Token};

#[test]
fn semicolon() {
    let source = ";";
    let tokens: Vec<Token> = Lex::new(source).filter_map(|token| token.ok()).collect();

    assert_eq!(
        Parse::new(tokens.as_slice()).parse_statement_node(),
        Ok(StatementNode {
            id: StatementId(Id(0)),
            kind: StatementKind::Semicolon,
        }),
    );
}

#[test]
fn r#break() {
    let source = "break";
    let tokens: Vec<Token> = Lex::new(source).filter_map(|token| token.ok()).collect();

    assert_eq!(
        Parse::new(tokens.as_slice()).parse_statement_node(),
        Ok(StatementNode {
            id: StatementId(Id(0)),
            kind: StatementKind::Break,
        }),
    );
}

#[test]
fn r#continue() {
    let source = "continue";
    let tokens: Vec<Token> = Lex::new(source).filter_map(|token| token.ok()).collect();

    assert_eq!(
        Parse::new(tokens.as_slice()).parse_statement_node(),
        Ok(StatementNode {
            id: StatementId(Id(0)),
            kind: StatementKind::Continue,
        }),
    );
}

#[test]
fn return_without_expression() {
    let source = "return;";
    let tokens: Vec<Token> = Lex::new(source).filter_map(|token| token.ok()).collect();

    assert_eq!(
        Parse::new(tokens.as_slice()).parse_statement_node(),
        Ok(StatementNode {
            id: StatementId(Id(0)),
            kind: StatementKind::Return(None),
        }),
    );
}

#[test]
fn return_with_expression() {
    let source = "return hello;";
    let lexer = Lex::new(source);
    let tokens: Vec<Token> = lexer.filter_map(|token| token.ok()).collect();

    assert_eq!(
        Parse::new(tokens.as_slice()).parse_statement_node(),
        Ok(StatementNode {
            id: StatementId(Id(0)),
            kind: StatementKind::Return(Some(ExpressionKind::Path(PathNode {
                segments: vec![IdentNode {
                    symbol: Symbol::intern("hello"),
                }],
            }))),
        }),
    );
}

#[test]
fn return_with_expressions() {
    let source = "return hello + my + world;";
    let lexer = Lex::new(source);
    let tokens: Vec<Token> = lexer.filter_map(|token| token.ok()).collect();

    assert_eq!(
        Parse::new(tokens.as_slice()).parse_statement_node(),
        Ok(StatementNode {
            id: StatementId(Id(0)),
            kind: StatementKind::Return(Some(ExpressionKind::Binary(BinaryExpressionNode {
                kind: BinaryOperatorKind::Add,
                lhs: Box::new(ExpressionKind::Binary(BinaryExpressionNode {
                    kind: BinaryOperatorKind::Add,
                    lhs: Box::new(ExpressionKind::Path(PathNode {
                        segments: vec![IdentNode {
                            symbol: Symbol::intern("hello"),
                        }],
                    })),
                    rhs: Box::new(ExpressionKind::Path(PathNode {
                        segments: vec![IdentNode {
                            symbol: Symbol::intern("my"),
                        }],
                    })),
                })),
                rhs: Box::new(ExpressionKind::Path(PathNode {
                    segments: vec![IdentNode {
                        symbol: Symbol::intern("world"),
                    }],
                })),
            }))),
        }),
    );
}

#[test]
fn let_with_nothing() {
    let source = "let foo;";
    let lexer = Lex::new(source);
    let tokens: Vec<Token> = lexer.filter_map(|token| token.ok()).collect();

    assert_eq!(
        Parse::new(tokens.as_slice()).parse_statement_node(),
        Ok(StatementNode {
            id: StatementId(Id(0)),
            kind: StatementKind::Let(Box::new(LetNode {
                immutability: ImmutabilityKind::Yes,
                pattern: PatternNode {
                    kind: PatternKind::Path(PathNode {
                        segments: vec![IdentNode {
                            symbol: Symbol::intern("foo"),
                        }],
                    }),
                },
                ty: None,
                value: None,
            })),
        }),
    );
}

#[test]
fn let_with_type() {
    let source = "let foo: Foo;";
    let lexer = Lex::new(source);
    let tokens: Vec<Token> = lexer.filter_map(|token| token.ok()).collect();

    assert_eq!(
        Parse::new(tokens.as_slice()).parse_statement_node(),
        Ok(StatementNode {
            id: StatementId(Id(0)),
            kind: StatementKind::Let(Box::new(LetNode {
                immutability: ImmutabilityKind::Yes,
                pattern: PatternNode {
                    kind: PatternKind::Path(PathNode {
                        segments: vec![IdentNode {
                            symbol: Symbol::intern("foo"),
                        }],
                    }),
                },
                ty: Some(TypeNode {
                    immutability: ImmutabilityKind::Yes,
                    kind: TypeKind::Path(PathNode {
                        segments: vec![IdentNode {
                            symbol: Symbol::intern("Foo"),
                        }],
                    }),
                }),
                value: None,
            })),
        }),
    );
}

#[test]
fn let_with_value() {
    let source = "let foo = bar;";
    let lexer = Lex::new(source);
    let tokens: Vec<Token> = lexer.filter_map(|token| token.ok()).collect();

    assert_eq!(
        Parse::new(tokens.as_slice()).parse_statement_node(),
        Ok(StatementNode {
            id: StatementId(Id(0)),
            kind: StatementKind::Let(Box::new(LetNode {
                immutability: ImmutabilityKind::Yes,
                pattern: PatternNode {
                    kind: PatternKind::Path(PathNode {
                        segments: vec![IdentNode {
                            symbol: Symbol::intern("foo"),
                        }],
                    }),
                },
                ty: None,
                value: Some(ExpressionKind::Path(PathNode {
                    segments: vec![IdentNode {
                        symbol: Symbol::intern("bar"),
                    }],
                })),
            })),
        }),
    );
}

#[test]
fn assign() {
    let source = "foo = bar;";
    let lexer = Lex::new(source);
    let tokens: Vec<Token> = lexer.filter_map(|token| token.ok()).collect();

    assert_eq!(
        Parse::new(tokens.as_slice()).parse_statement_node(),
        Ok(StatementNode {
            id: StatementId(Id(0)),
            kind: StatementKind::Assign(Box::new(AssignNode {
                kind: AssignKind::Assign,
                lhs: ExpressionKind::Path(PathNode {
                    segments: vec![IdentNode {
                        symbol: Symbol::intern("foo"),
                    }],
                }),
                rhs: ExpressionKind::Path(PathNode {
                    segments: vec![IdentNode {
                        symbol: Symbol::intern("bar"),
                    }],
                }),
            })),
        }),
    );
}

#[test]
fn add_assign() {
    let source = "foo += bar;";
    let lexer = Lex::new(source);
    let tokens: Vec<Token> = lexer.filter_map(|token| token.ok()).collect();

    assert_eq!(
        Parse::new(tokens.as_slice()).parse_statement_node(),
        Ok(StatementNode {
            id: StatementId(Id(0)),
            kind: StatementKind::Assign(Box::new(AssignNode {
                kind: AssignKind::Add,
                lhs: ExpressionKind::Path(PathNode {
                    segments: vec![IdentNode {
                        symbol: Symbol::intern("foo"),
                    }],
                }),
                rhs: ExpressionKind::Path(PathNode {
                    segments: vec![IdentNode {
                        symbol: Symbol::intern("bar"),
                    }],
                }),
            })),
        }),
    );
}
