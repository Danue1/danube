use crate::Parse;
use danube_ast::{
    BlockNode, ConditionBranch, ConditionNode, ExpressionKind, ForNode, IdentNode, LoopNode,
    MatchNode, PathNode, WhileNode,
};
use danube_lex::Lex;
use danube_token::{LiteralKind, Symbol, Token};

#[test]
fn ident() {
    let source = "foo";
    let tokens: Vec<Token> = Lex::new(source).filter_map(|token| token.ok()).collect();

    assert_eq!(
        Parse::new(tokens.as_slice()).parse_expression_kind(),
        Ok(ExpressionKind::Path(PathNode {
            idents: vec![IdentNode {
                symbol: Symbol::intern("foo")
            }]
        }))
    );
}

#[test]
fn path() {
    let source = "foo::bar";
    let tokens: Vec<Token> = Lex::new(source).filter_map(|token| token.ok()).collect();

    assert_eq!(
        Parse::new(tokens.as_slice()).parse_expression_kind(),
        Ok(ExpressionKind::Path(PathNode {
            idents: vec![
                IdentNode {
                    symbol: Symbol::intern("foo")
                },
                IdentNode {
                    symbol: Symbol::intern("bar")
                },
            ]
        }))
    );
}

#[test]
fn add() {
    let source = "+foo";
    let tokens: Vec<Token> = Lex::new(source).filter_map(|token| token.ok()).collect();

    assert_eq!(
        Parse::new(tokens.as_slice()).parse_expression_kind(),
        Ok(ExpressionKind::Path(PathNode {
            idents: vec![IdentNode {
                symbol: Symbol::intern("foo")
            }]
        }))
    );
}

#[test]
fn negate() {
    let source = "-foo";
    let tokens: Vec<Token> = Lex::new(source).filter_map(|token| token.ok()).collect();

    assert_eq!(
        Parse::new(tokens.as_slice()).parse_expression_kind(),
        Ok(ExpressionKind::Negate(Box::new(ExpressionKind::Path(
            PathNode {
                idents: vec![IdentNode {
                    symbol: Symbol::intern("foo")
                }]
            }
        ))))
    );
}

#[test]
fn not() {
    let source = "!foo";
    let tokens: Vec<Token> = Lex::new(source).filter_map(|token| token.ok()).collect();

    assert_eq!(
        Parse::new(tokens.as_slice()).parse_expression_kind(),
        Ok(ExpressionKind::Not(Box::new(ExpressionKind::Path(
            PathNode {
                idents: vec![IdentNode {
                    symbol: Symbol::intern("foo")
                }]
            }
        ))))
    );
}

#[test]
fn bit_not() {
    let source = "~foo";
    let tokens: Vec<Token> = Lex::new(source).filter_map(|token| token.ok()).collect();

    assert_eq!(
        Parse::new(tokens.as_slice()).parse_expression_kind(),
        Ok(ExpressionKind::BitNot(Box::new(ExpressionKind::Path(
            PathNode {
                idents: vec![IdentNode {
                    symbol: Symbol::intern("foo")
                }]
            }
        ))))
    );
}

#[test]
fn char() {
    let source = "'a'";
    let tokens: Vec<Token> = Lex::new(source).filter_map(|token| token.ok()).collect();

    assert_eq!(
        Parse::new(tokens.as_slice()).parse_expression_kind(),
        Ok(ExpressionKind::Literal(
            Symbol::intern("a"),
            LiteralKind::Char
        ))
    );
}

#[test]
fn integer() {
    let source = "123";
    let tokens: Vec<Token> = Lex::new(source).filter_map(|token| token.ok()).collect();

    assert_eq!(
        Parse::new(tokens.as_slice()).parse_expression_kind(),
        Ok(ExpressionKind::Literal(
            Symbol::intern("123"),
            LiteralKind::Integer
        ))
    );
}

#[test]
fn float() {
    let source = "123.456";
    let tokens: Vec<Token> = Lex::new(source).filter_map(|token| token.ok()).collect();

    assert_eq!(
        Parse::new(tokens.as_slice()).parse_expression_kind(),
        Ok(ExpressionKind::Literal(
            Symbol::intern("123.456"),
            LiteralKind::Float
        ))
    );
}

#[test]
fn string() {
    let source = "\"foo\"";
    let tokens: Vec<Token> = Lex::new(source).filter_map(|token| token.ok()).collect();

    assert_eq!(
        Parse::new(tokens.as_slice()).parse_expression_kind(),
        Ok(ExpressionKind::Literal(
            Symbol::intern("foo"),
            LiteralKind::String
        ))
    );
}

#[test]
fn conditional_without_else() {
    let source = "if hello { }";
    let tokens: Vec<Token> = Lex::new(source).filter_map(|token| token.ok()).collect();

    assert_eq!(
        Parse::new(tokens.as_slice()).parse_expression_kind(),
        Ok(ExpressionKind::Conditional(ConditionNode {
            branches: vec![ConditionBranch {
                expression: Box::new(ExpressionKind::Path(PathNode {
                    idents: vec![IdentNode {
                        symbol: Symbol::intern("hello")
                    }]
                })),
                block: BlockNode { statements: vec![] }
            }],
            other: None,
        }))
    );
}

#[test]
fn conditional_with_else() {
    let source = "if hello { } else { }";
    let tokens: Vec<Token> = Lex::new(source).filter_map(|token| token.ok()).collect();

    assert_eq!(
        Parse::new(tokens.as_slice()).parse_expression_kind(),
        Ok(ExpressionKind::Conditional(ConditionNode {
            branches: vec![ConditionBranch {
                expression: Box::new(ExpressionKind::Path(PathNode {
                    idents: vec![IdentNode {
                        symbol: Symbol::intern("hello")
                    }]
                })),
                block: BlockNode { statements: vec![] }
            }],
            other: Some(BlockNode { statements: vec![] }),
        }))
    );
}

#[test]
fn r#loop() {
    let source = "loop { }";
    let tokens: Vec<Token> = Lex::new(source).filter_map(|token| token.ok()).collect();

    assert_eq!(
        Parse::new(tokens.as_slice()).parse_expression_kind(),
        Ok(ExpressionKind::Loop(LoopNode {
            block: BlockNode { statements: vec![] }
        }))
    );
}

#[test]
fn r#while() {
    let source = "while hello { }";
    let tokens: Vec<Token> = Lex::new(source).filter_map(|token| token.ok()).collect();

    assert_eq!(
        Parse::new(tokens.as_slice()).parse_expression_kind(),
        Ok(ExpressionKind::While(WhileNode {
            branch: ConditionBranch {
                expression: Box::new(ExpressionKind::Path(PathNode {
                    idents: vec![IdentNode {
                        symbol: Symbol::intern("hello")
                    }]
                })),
                block: BlockNode { statements: vec![] }
            },
        }))
    );
}

#[test]
#[ignore]
fn r#for() {
    let source = "for foo in bar { }";
    let tokens: Vec<Token> = Lex::new(source).filter_map(|token| token.ok()).collect();

    assert_eq!(
        Parse::new(tokens.as_slice()).parse_expression_kind(),
        Ok(ExpressionKind::Loop(LoopNode {
            block: BlockNode { statements: vec![] }
        }))
    );
}

#[test]
#[ignore]
fn r#match() {
    let source = "match foo { 1 => { } }";
    let tokens: Vec<Token> = Lex::new(source).filter_map(|token| token.ok()).collect();

    assert_eq!(
        Parse::new(tokens.as_slice()).parse_expression_kind(),
        Ok(ExpressionKind::Loop(LoopNode {
            block: BlockNode { statements: vec![] }
        }))
    );
}
