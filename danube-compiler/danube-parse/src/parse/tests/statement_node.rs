use crate::Parse;
use danube_ast::{
    BinaryExpressionNode, BinaryOperatorKind, ExpressionKind, Id, IdentNode, PathNode, StatementId,
    StatementKind, StatementNode,
};
use danube_lex::Lex;
use danube_token::{Symbol, Token};

#[test]
fn statement_semicolon() {
    let source = ";";
    let tokens: Vec<Token> = Lex::new(source).filter_map(|token| token.ok()).collect();

    assert_eq!(
        Parse::new(tokens.as_slice()).parse_statement_node(),
        Ok(StatementNode {
            id: StatementId(Id(0)),
            kind: StatementKind::Semicolon
        })
    );
}

#[test]
fn statement_break() {
    let source = "break";
    let tokens: Vec<Token> = Lex::new(source).filter_map(|token| token.ok()).collect();

    assert_eq!(
        Parse::new(tokens.as_slice()).parse_statement_node(),
        Ok(StatementNode {
            id: StatementId(Id(0)),
            kind: StatementKind::Break
        })
    );
}

#[test]
fn statement_continue() {
    let source = "continue";
    let tokens: Vec<Token> = Lex::new(source).filter_map(|token| token.ok()).collect();

    assert_eq!(
        Parse::new(tokens.as_slice()).parse_statement_node(),
        Ok(StatementNode {
            id: StatementId(Id(0)),
            kind: StatementKind::Continue
        })
    );
}

#[test]
fn statement_return_without_expression() {
    let source = "return;";
    let tokens: Vec<Token> = Lex::new(source).filter_map(|token| token.ok()).collect();

    assert_eq!(
        Parse::new(tokens.as_slice()).parse_statement_node(),
        Ok(StatementNode {
            id: StatementId(Id(0)),
            kind: StatementKind::Return(None)
        })
    );
}

#[test]
fn statement_return_with_expression() {
    let source = "return hello;";
    let lexer = Lex::new(source);
    let tokens: Vec<Token> = lexer.filter_map(|token| token.ok()).collect();

    assert_eq!(
        Parse::new(tokens.as_slice()).parse_statement_node(),
        Ok(StatementNode {
            id: StatementId(Id(0)),
            kind: StatementKind::Return(Some(ExpressionKind::Path(PathNode {
                idents: vec![IdentNode {
                    symbol: Symbol::intern("hello")
                }]
            })))
        })
    );
}

#[test]
fn statement_return_with_expressions() {
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
                        idents: vec![IdentNode {
                            symbol: Symbol::intern("hello"),
                        }]
                    })),
                    rhs: Box::new(ExpressionKind::Path(PathNode {
                        idents: vec![IdentNode {
                            symbol: Symbol::intern("my"),
                        }]
                    })),
                })),
                rhs: Box::new(ExpressionKind::Path(PathNode {
                    idents: vec![IdentNode {
                        symbol: Symbol::intern("world"),
                    }]
                })),
            })))
        })
    );
}
