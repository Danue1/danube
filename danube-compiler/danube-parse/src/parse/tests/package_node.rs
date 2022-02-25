use crate::Parse;
use danube_ast::{
    AttributeNode, IdentNode, PackageNode, PathNode, DUMMY_ATTRIBUTE_ID, DUMMY_NODE_ID,
};
use danube_lex::Lex;
use danube_token::{Symbol, Token};

#[test]
fn attribute() {
    let source = "#![hello]";
    let tokens: Vec<Token> = Lex::new(source).filter_map(|token| token.ok()).collect();

    assert_eq!(
        Parse::new(tokens.as_slice()).parse_package_node(),
        Ok(PackageNode {
            id: DUMMY_NODE_ID,
            attributes: vec![AttributeNode {
                id: DUMMY_ATTRIBUTE_ID,
                path: PathNode {
                    segments: vec![IdentNode {
                        id: DUMMY_NODE_ID,
                        symbol: Symbol::intern("hello")
                    }]
                },
                args: vec![],
            }],
            items: vec![],
        }),
    );
}

#[test]
fn attributes() {
    let source = "#![hello]\
        #![hello]";
    let tokens: Vec<Token> = Lex::new(source).filter_map(|token| token.ok()).collect();

    assert_eq!(
        Parse::new(tokens.as_slice()).parse_package_node(),
        Ok(PackageNode {
            id: DUMMY_NODE_ID,
            attributes: vec![
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
            ],
            items: vec![],
        }),
    );
}
