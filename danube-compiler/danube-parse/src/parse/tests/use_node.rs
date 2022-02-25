use crate::Parse;
use danube_ast::{IdentNode, PathNode, UseNode, DUMMY_NODE_ID};
use danube_lex::Lex;
use danube_token::{Symbol, Token};

#[test]
fn one() {
    let source = "one;";
    let tokens: Vec<Token> = Lex::new(source).filter_map(|token| token.ok()).collect();

    assert_eq!(
        Parse::new(tokens.as_slice()).parse_use_node(),
        Ok(UseNode {
            path: PathNode {
                segments: vec![IdentNode {
                    id: DUMMY_NODE_ID,
                    symbol: Symbol::intern("one"),
                }],
            },
        }),
    );
}

#[test]
fn two() {
    let source = "one::two;";
    let tokens: Vec<Token> = Lex::new(source).filter_map(|token| token.ok()).collect();

    dbg!(&tokens);

    assert_eq!(
        Parse::new(tokens.as_slice()).parse_use_node(),
        Ok(UseNode {
            path: PathNode {
                segments: vec![
                    IdentNode {
                        id: DUMMY_NODE_ID,
                        symbol: Symbol::intern("one"),
                    },
                    IdentNode {
                        id: DUMMY_NODE_ID,
                        symbol: Symbol::intern("two"),
                    },
                ],
            },
        }),
    );
}
