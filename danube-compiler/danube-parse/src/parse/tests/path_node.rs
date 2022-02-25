use crate::Parse;
use danube_ast::{IdentNode, PathNode, DUMMY_NODE_ID};
use danube_lex::Lex;
use danube_token::{Symbol, Token};

#[test]
fn one() {
    let source = "one";
    let tokens: Vec<Token> = Lex::new(source).filter_map(|token| token.ok()).collect();

    assert_eq!(
        Parse::new(tokens.as_slice()).parse_path_node(),
        Ok(Some(PathNode {
            segments: vec![IdentNode {
                id: DUMMY_NODE_ID,
                symbol: Symbol::intern("one"),
            }],
        })),
    );
}

#[test]
fn onw_two() {
    let source = "one::two";
    let tokens: Vec<Token> = Lex::new(source).filter_map(|token| token.ok()).collect();

    assert_eq!(
        Parse::new(tokens.as_slice()).parse_path_node(),
        Ok(Some(PathNode {
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
        })),
    );
}
