use crate::parse::attribute_node::{ItemAttributeNodeList, PackageAttributeNodeList};
use crate::{Context, Parse};
use danube_ast::{
    AttributeNode, ExpressionKind, ExpressionNode, IdentNode, PathNode, DUMMY_ATTRIBUTE_ID,
    DUMMY_NODE_ID,
};
use danube_lex::Lex;
use danube_token::{Symbol, Token};

#[test]
fn package_attribute() {
    let source = "#![hello]";
    let tokens: Vec<Token> = Lex::new(source).filter_map(|token| token.ok()).collect();

    assert_eq!(
        PackageAttributeNodeList::parse(&mut Context::new(tokens.as_slice())),
        Ok(vec![AttributeNode {
            id: DUMMY_ATTRIBUTE_ID,
            path: PathNode {
                segments: vec![IdentNode {
                    id: DUMMY_NODE_ID,
                    symbol: Symbol::intern("hello"),
                }],
            },
            args: vec![],
        }]),
    );
}

#[test]
fn package_attributes() {
    let source = "#![hello] #![hello]";
    let tokens: Vec<Token> = Lex::new(source).filter_map(|token| token.ok()).collect();

    assert_eq!(
        PackageAttributeNodeList::parse(&mut Context::new(tokens.as_slice())),
        Ok(vec![
            AttributeNode {
                id: DUMMY_ATTRIBUTE_ID,
                path: PathNode {
                    segments: vec![IdentNode {
                        id: DUMMY_NODE_ID,
                        symbol: Symbol::intern("hello"),
                    }],
                },
                args: vec![],
            },
            AttributeNode {
                id: DUMMY_ATTRIBUTE_ID,
                path: PathNode {
                    segments: vec![IdentNode {
                        id: DUMMY_NODE_ID,
                        symbol: Symbol::intern("hello"),
                    }]
                },
                args: vec![],
            },
        ]),
    );
}

#[test]
fn item_attribute() {
    let source = "#[hello]";
    let tokens: Vec<Token> = Lex::new(source).filter_map(|token| token.ok()).collect();

    assert_eq!(
        ItemAttributeNodeList::parse(&mut Context::new(tokens.as_slice())),
        Ok(vec![AttributeNode {
            id: DUMMY_ATTRIBUTE_ID,
            path: PathNode {
                segments: vec![IdentNode {
                    id: DUMMY_NODE_ID,
                    symbol: Symbol::intern("hello"),
                }],
            },
            args: vec![],
        }]),
    );
}

#[test]
fn item_attributes() {
    let source = "#[hello] #[hello]";
    let tokens: Vec<Token> = Lex::new(source).filter_map(|token| token.ok()).collect();

    assert_eq!(
        ItemAttributeNodeList::parse(&mut Context::new(tokens.as_slice())),
        Ok(vec![
            AttributeNode {
                id: DUMMY_ATTRIBUTE_ID,
                path: PathNode {
                    segments: vec![IdentNode {
                        id: DUMMY_NODE_ID,
                        symbol: Symbol::intern("hello"),
                    }],
                },
                args: vec![],
            },
            AttributeNode {
                id: DUMMY_ATTRIBUTE_ID,
                path: PathNode {
                    segments: vec![IdentNode {
                        id: DUMMY_NODE_ID,
                        symbol: Symbol::intern("hello"),
                    }],
                },
                args: vec![],
            },
        ]),
    );
}

#[test]
fn item_attribute_with_argument() {
    let source = "#[hello(foo)]";
    let tokens: Vec<Token> = Lex::new(source).filter_map(|token| token.ok()).collect();

    assert_eq!(
        ItemAttributeNodeList::parse(&mut Context::new(tokens.as_slice())),
        Ok(vec![AttributeNode {
            id: DUMMY_ATTRIBUTE_ID,
            path: PathNode {
                segments: vec![IdentNode {
                    id: DUMMY_NODE_ID,
                    symbol: Symbol::intern("hello"),
                }],
            },
            args: vec![(
                IdentNode {
                    id: DUMMY_NODE_ID,
                    symbol: Symbol::intern("foo"),
                },
                None,
            )],
        }]),
    );
}

#[test]
fn item_attribute_with_argument_and_expression() {
    let source = "#[hello(foo = bar)]";
    let tokens: Vec<Token> = Lex::new(source).filter_map(|token| token.ok()).collect();

    assert_eq!(
        ItemAttributeNodeList::parse(&mut Context::new(tokens.as_slice())),
        Ok(vec![AttributeNode {
            id: DUMMY_ATTRIBUTE_ID,
            path: PathNode {
                segments: vec![IdentNode {
                    id: DUMMY_NODE_ID,
                    symbol: Symbol::intern("hello"),
                }],
            },
            args: vec![(
                IdentNode {
                    id: DUMMY_NODE_ID,
                    symbol: Symbol::intern("foo"),
                },
                Some(ExpressionNode {
                    id: DUMMY_NODE_ID,
                    kind: ExpressionKind::Path(PathNode {
                        segments: vec![IdentNode {
                            id: DUMMY_NODE_ID,
                            symbol: Symbol::intern("bar"),
                        }],
                    }),
                }),
            )],
        }]),
    );
}
