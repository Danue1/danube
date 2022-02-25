use crate::Parse;
use danube_ast::{BlockNode, DUMMY_NODE_ID};
use danube_lex::Lex;
use danube_token::Token;

#[test]
fn empty_block() {
    let source = "{ }";
    let tokens: Vec<Token> = Lex::new(source).filter_map(|token| token.ok()).collect();

    assert_eq!(
        Parse::new(tokens.as_slice()).parse_block_node(),
        Ok(BlockNode {
            id: DUMMY_NODE_ID,
            statements: vec![]
        }),
    );
}
