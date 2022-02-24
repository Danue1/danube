use crate::Parse;
use danube_ast::{
    ConstantNode, ExpressionKind, IdentNode, ImmutablityKind, PathNode, PatternKind, PatternNode,
    TypeKind, TypeNode,
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
                kind: PatternKind::Path(PathNode {
                    idents: vec![IdentNode {
                        symbol: Symbol::intern("foo")
                    }]
                }),
            },
            ty: TypeNode {
                immutablity: ImmutablityKind::Yes,
                kind: TypeKind::Path(PathNode {
                    idents: vec![IdentNode {
                        symbol: Symbol::intern("bar")
                    }]
                }),
            },
            expression: None,
        })
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
                kind: PatternKind::Path(PathNode {
                    idents: vec![IdentNode {
                        symbol: Symbol::intern("foo")
                    }]
                }),
            },
            ty: TypeNode {
                immutablity: ImmutablityKind::Yes,
                kind: TypeKind::Path(PathNode {
                    idents: vec![IdentNode {
                        symbol: Symbol::intern("bar")
                    }]
                }),
            },
            expression: Some(ExpressionKind::Path(PathNode {
                idents: vec![IdentNode {
                    symbol: Symbol::intern("baz")
                }],
            })),
        })
    );
}
