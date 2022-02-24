use crate::Parse;
use danube_ast::{AttributeNode, ExpressionKind, IdentNode, PathNode};
use danube_lex::Lex;
use danube_token::{Symbol, Token};

#[test]
fn package_attribute() {
    let source = "#![hello]";
    let tokens: Vec<Token> = Lex::new(source).filter_map(|token| token.ok()).collect();

    assert_eq!(
        Parse::new(tokens.as_slice()).parse_package_attributes(),
        Ok(vec![AttributeNode {
            path: PathNode {
                segments: vec![IdentNode {
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
        Parse::new(tokens.as_slice()).parse_package_attributes(),
        Ok(vec![
            AttributeNode {
                path: PathNode {
                    segments: vec![IdentNode {
                        symbol: Symbol::intern("hello"),
                    }],
                },
                args: vec![],
            },
            AttributeNode {
                path: PathNode {
                    segments: vec![IdentNode {
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
        Parse::new(tokens.as_slice()).parse_item_attributes(),
        Ok(vec![AttributeNode {
            path: PathNode {
                segments: vec![IdentNode {
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
        Parse::new(tokens.as_slice()).parse_item_attributes(),
        Ok(vec![
            AttributeNode {
                path: PathNode {
                    segments: vec![IdentNode {
                        symbol: Symbol::intern("hello"),
                    }],
                },
                args: vec![],
            },
            AttributeNode {
                path: PathNode {
                    segments: vec![IdentNode {
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
        Parse::new(tokens.as_slice()).parse_item_attributes(),
        Ok(vec![AttributeNode {
            path: PathNode {
                segments: vec![IdentNode {
                    symbol: Symbol::intern("hello"),
                }],
            },
            args: vec![(
                IdentNode {
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
        Parse::new(tokens.as_slice()).parse_item_attributes(),
        Ok(vec![AttributeNode {
            path: PathNode {
                segments: vec![IdentNode {
                    symbol: Symbol::intern("hello"),
                }],
            },
            args: vec![(
                IdentNode {
                    symbol: Symbol::intern("foo"),
                },
                Some(ExpressionKind::Path(PathNode {
                    segments: vec![IdentNode {
                        symbol: Symbol::intern("bar"),
                    }],
                })),
            )],
        }]),
    );
}
