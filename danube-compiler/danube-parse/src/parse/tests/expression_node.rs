use crate::Parse;
use danube_ast::{
    BlockNode, ConditionBranch, ConditionNode, ExpressionKind, ExpressionNode, ForNode, IdentNode,
    LoopNode, MatchBranch, MatchNode, PathNode, PatternKind, PatternNode, WhileNode, DUMMY_NODE_ID,
};
use danube_lex::Lex;
use danube_token::{LiteralKind, Symbol, Token};

#[test]
fn ident() {
    let source = "foo";
    let tokens: Vec<Token> = Lex::new(source).filter_map(|token| token.ok()).collect();

    assert_eq!(
        Parse::new(tokens.as_slice()).parse_expression_node(),
        Ok(ExpressionNode {
            id: DUMMY_NODE_ID,
            kind: ExpressionKind::Path(PathNode {
                segments: vec![IdentNode {
                    id: DUMMY_NODE_ID,
                    symbol: Symbol::intern("foo"),
                }],
            }),
        }),
    );
}

#[test]
fn path() {
    let source = "foo::bar";
    let tokens: Vec<Token> = Lex::new(source).filter_map(|token| token.ok()).collect();

    assert_eq!(
        Parse::new(tokens.as_slice()).parse_expression_node(),
        Ok(ExpressionNode {
            id: DUMMY_NODE_ID,
            kind: ExpressionKind::Path(PathNode {
                segments: vec![
                    IdentNode {
                        id: DUMMY_NODE_ID,
                        symbol: Symbol::intern("foo"),
                    },
                    IdentNode {
                        id: DUMMY_NODE_ID,
                        symbol: Symbol::intern("bar"),
                    },
                ],
            }),
        }),
    );
}

#[test]
fn add() {
    let source = "+foo";
    let tokens: Vec<Token> = Lex::new(source).filter_map(|token| token.ok()).collect();

    assert_eq!(
        Parse::new(tokens.as_slice()).parse_expression_node(),
        Ok(ExpressionNode {
            id: DUMMY_NODE_ID,
            kind: ExpressionKind::Path(PathNode {
                segments: vec![IdentNode {
                    id: DUMMY_NODE_ID,
                    symbol: Symbol::intern("foo"),
                }],
            }),
        }),
    );
}

#[test]
fn negate() {
    let source = "-foo";
    let tokens: Vec<Token> = Lex::new(source).filter_map(|token| token.ok()).collect();

    assert_eq!(
        Parse::new(tokens.as_slice()).parse_expression_node(),
        Ok(ExpressionNode {
            id: DUMMY_NODE_ID,
            kind: ExpressionKind::Negate(Box::new(ExpressionNode {
                id: DUMMY_NODE_ID,
                kind: ExpressionKind::Path(PathNode {
                    segments: vec![IdentNode {
                        id: DUMMY_NODE_ID,
                        symbol: Symbol::intern("foo"),
                    }],
                }),
            })),
        }),
    );
}

#[test]
fn not() {
    let source = "!foo";
    let tokens: Vec<Token> = Lex::new(source).filter_map(|token| token.ok()).collect();

    assert_eq!(
        Parse::new(tokens.as_slice()).parse_expression_node(),
        Ok(ExpressionNode {
            id: DUMMY_NODE_ID,
            kind: ExpressionKind::Not(Box::new(ExpressionNode {
                id: DUMMY_NODE_ID,
                kind: ExpressionKind::Path(PathNode {
                    segments: vec![IdentNode {
                        id: DUMMY_NODE_ID,
                        symbol: Symbol::intern("foo"),
                    }],
                }),
            })),
        }),
    );
}

#[test]
fn bit_not() {
    let source = "~foo";
    let tokens: Vec<Token> = Lex::new(source).filter_map(|token| token.ok()).collect();

    assert_eq!(
        Parse::new(tokens.as_slice()).parse_expression_node(),
        Ok(ExpressionNode {
            id: DUMMY_NODE_ID,
            kind: ExpressionKind::BitNot(Box::new(ExpressionNode {
                id: DUMMY_NODE_ID,
                kind: ExpressionKind::Path(PathNode {
                    segments: vec![IdentNode {
                        id: DUMMY_NODE_ID,
                        symbol: Symbol::intern("foo"),
                    }],
                }),
            })),
        }),
    );
}

#[test]
fn char() {
    let source = "'a'";
    let tokens: Vec<Token> = Lex::new(source).filter_map(|token| token.ok()).collect();

    assert_eq!(
        Parse::new(tokens.as_slice()).parse_expression_node(),
        Ok(ExpressionNode {
            id: DUMMY_NODE_ID,
            kind: ExpressionKind::Literal(Symbol::intern("a"), LiteralKind::Char),
        }),
    );
}

#[test]
fn integer() {
    let source = "123";
    let tokens: Vec<Token> = Lex::new(source).filter_map(|token| token.ok()).collect();

    assert_eq!(
        Parse::new(tokens.as_slice()).parse_expression_node(),
        Ok(ExpressionNode {
            id: DUMMY_NODE_ID,
            kind: ExpressionKind::Literal(Symbol::intern("123"), LiteralKind::Integer),
        }),
    );
}

#[test]
fn float() {
    let source = "123.456";
    let tokens: Vec<Token> = Lex::new(source).filter_map(|token| token.ok()).collect();

    assert_eq!(
        Parse::new(tokens.as_slice()).parse_expression_node(),
        Ok(ExpressionNode {
            id: DUMMY_NODE_ID,
            kind: ExpressionKind::Literal(Symbol::intern("123.456"), LiteralKind::Float),
        }),
    );
}

#[test]
fn string() {
    let source = "\"foo\"";
    let tokens: Vec<Token> = Lex::new(source).filter_map(|token| token.ok()).collect();

    assert_eq!(
        Parse::new(tokens.as_slice()).parse_expression_node(),
        Ok(ExpressionNode {
            id: DUMMY_NODE_ID,
            kind: ExpressionKind::Literal(Symbol::intern("foo"), LiteralKind::String),
        }),
    );
}

#[test]
fn conditional_without_else() {
    let source = "if hello { }";
    let tokens: Vec<Token> = Lex::new(source).filter_map(|token| token.ok()).collect();

    assert_eq!(
        Parse::new(tokens.as_slice()).parse_expression_node(),
        Ok(ExpressionNode {
            id: DUMMY_NODE_ID,
            kind: ExpressionKind::Conditional(ConditionNode {
                branches: vec![ConditionBranch {
                    expression: Box::new(ExpressionNode {
                        id: DUMMY_NODE_ID,
                        kind: ExpressionKind::Path(PathNode {
                            segments: vec![IdentNode {
                                id: DUMMY_NODE_ID,
                                symbol: Symbol::intern("hello"),
                            }],
                        }),
                    }),
                    block: BlockNode {
                        id: DUMMY_NODE_ID,
                        statements: vec![]
                    },
                }],
                other: None,
            }),
        }),
    );
}

#[test]
fn conditional_with_else() {
    let source = "if hello { } else { }";
    let tokens: Vec<Token> = Lex::new(source).filter_map(|token| token.ok()).collect();

    assert_eq!(
        Parse::new(tokens.as_slice()).parse_expression_node(),
        Ok(ExpressionNode {
            id: DUMMY_NODE_ID,
            kind: ExpressionKind::Conditional(ConditionNode {
                branches: vec![ConditionBranch {
                    expression: Box::new(ExpressionNode {
                        id: DUMMY_NODE_ID,
                        kind: ExpressionKind::Path(PathNode {
                            segments: vec![IdentNode {
                                id: DUMMY_NODE_ID,
                                symbol: Symbol::intern("hello"),
                            }],
                        }),
                    }),
                    block: BlockNode {
                        id: DUMMY_NODE_ID,
                        statements: vec![]
                    },
                }],
                other: Some(BlockNode {
                    id: DUMMY_NODE_ID,
                    statements: vec![]
                }),
            }),
        }),
    );
}

#[test]
fn r#loop() {
    let source = "loop { }";
    let tokens: Vec<Token> = Lex::new(source).filter_map(|token| token.ok()).collect();

    assert_eq!(
        Parse::new(tokens.as_slice()).parse_expression_node(),
        Ok(ExpressionNode {
            id: DUMMY_NODE_ID,
            kind: ExpressionKind::Loop(LoopNode {
                block: BlockNode {
                    id: DUMMY_NODE_ID,
                    statements: vec![]
                },
            }),
        }),
    );
}

#[test]
fn r#while() {
    let source = "while hello { }";
    let tokens: Vec<Token> = Lex::new(source).filter_map(|token| token.ok()).collect();

    assert_eq!(
        Parse::new(tokens.as_slice()).parse_expression_node(),
        Ok(ExpressionNode {
            id: DUMMY_NODE_ID,
            kind: ExpressionKind::While(WhileNode {
                branch: ConditionBranch {
                    expression: Box::new(ExpressionNode {
                        id: DUMMY_NODE_ID,
                        kind: ExpressionKind::Path(PathNode {
                            segments: vec![IdentNode {
                                id: DUMMY_NODE_ID,
                                symbol: Symbol::intern("hello"),
                            }],
                        }),
                    }),
                    block: BlockNode {
                        id: DUMMY_NODE_ID,
                        statements: vec![]
                    },
                },
            }),
        }),
    );
}

#[test]
fn r#for() {
    let source = "for foo in bar { }";
    let tokens: Vec<Token> = Lex::new(source).filter_map(|token| token.ok()).collect();

    assert_eq!(
        Parse::new(tokens.as_slice()).parse_expression_node(),
        Ok(ExpressionNode {
            id: DUMMY_NODE_ID,
            kind: ExpressionKind::For(ForNode {
                pattern: PatternNode {
                    id: DUMMY_NODE_ID,
                    kind: PatternKind::Path(PathNode {
                        segments: vec![IdentNode {
                            id: DUMMY_NODE_ID,
                            symbol: Symbol::intern("foo"),
                        }],
                    }),
                },
                iter: Box::new(ExpressionNode {
                    id: DUMMY_NODE_ID,
                    kind: ExpressionKind::Path(PathNode {
                        segments: vec![IdentNode {
                            id: DUMMY_NODE_ID,
                            symbol: Symbol::intern("bar"),
                        }],
                    }),
                }),
                block: BlockNode {
                    id: DUMMY_NODE_ID,
                    statements: vec![]
                },
            }),
        }),
    );
}

#[test]
fn r#match() {
    let source = "match foo { 1 => { } }";
    let tokens: Vec<Token> = Lex::new(source).filter_map(|token| token.ok()).collect();

    assert_eq!(
        Parse::new(tokens.as_slice()).parse_expression_node(),
        Ok(ExpressionNode {
            id: DUMMY_NODE_ID,
            kind: ExpressionKind::Match(MatchNode {
                expression: Box::new(ExpressionNode {
                    id: DUMMY_NODE_ID,
                    kind: ExpressionKind::Path(PathNode {
                        segments: vec![IdentNode {
                            id: DUMMY_NODE_ID,
                            symbol: Symbol::intern("foo"),
                        }],
                    }),
                }),
                branches: vec![MatchBranch {
                    pattern: PatternNode {
                        id: DUMMY_NODE_ID,
                        kind: PatternKind::Literal(Symbol::intern("1"), LiteralKind::Integer,),
                    },
                    block: BlockNode {
                        id: DUMMY_NODE_ID,
                        statements: vec![]
                    },
                }],
            }),
        }),
    );
}
