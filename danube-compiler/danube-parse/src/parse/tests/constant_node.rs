use crate::Parse;
use danube_ast::{
    ConstantNode, ExpressionKind, ExpressionNode, IdentNode, ImmutabilityKind, PathNode,
    PatternKind, PatternNode, TypeKind, TypeNode, DUMMY_NODE_ID,
};
use danube_lex::Lex;
use danube_token::{Symbol, Token};

#[test]
fn without_expression() {
    let source = "foo: bar;";
    let tokens: Vec<Token> = Lex::new(source).filter_map(|token| token.ok()).collect();

    assert_eq!(
        Parse::new(tokens.as_slice()).parse_constant_node(),
        Ok(ConstantNode {
            pattern: PatternNode {
                id: DUMMY_NODE_ID,
                kind: PatternKind::Path(PathNode {
                    segments: vec![IdentNode {
                        id: DUMMY_NODE_ID,
                        symbol: Symbol::intern("foo"),
                    }],
                }),
            },
            ty: TypeNode {
                id: DUMMY_NODE_ID,
                immutability: ImmutabilityKind::Yes,
                kind: TypeKind::Path(PathNode {
                    segments: vec![IdentNode {
                        id: DUMMY_NODE_ID,
                        symbol: Symbol::intern("bar"),
                    }],
                }),
            },
            expression: None,
        }),
    );
}

#[test]
fn with_expression() {
    let source = "foo: bar = baz;";
    let tokens: Vec<Token> = Lex::new(source).filter_map(|token| token.ok()).collect();

    assert_eq!(
        Parse::new(tokens.as_slice()).parse_constant_node(),
        Ok(ConstantNode {
            pattern: PatternNode {
                id: DUMMY_NODE_ID,
                kind: PatternKind::Path(PathNode {
                    segments: vec![IdentNode {
                        id: DUMMY_NODE_ID,
                        symbol: Symbol::intern("foo"),
                    }],
                }),
            },
            ty: TypeNode {
                id: DUMMY_NODE_ID,
                immutability: ImmutabilityKind::Yes,
                kind: TypeKind::Path(PathNode {
                    segments: vec![IdentNode {
                        id: DUMMY_NODE_ID,
                        symbol: Symbol::intern("bar"),
                    }],
                }),
            },
            expression: Some(ExpressionNode {
                id: DUMMY_NODE_ID,
                kind: ExpressionKind::Path(PathNode {
                    segments: vec![IdentNode {
                        id: DUMMY_NODE_ID,
                        symbol: Symbol::intern("baz"),
                    }],
                })
            }),
        }),
    );
}
