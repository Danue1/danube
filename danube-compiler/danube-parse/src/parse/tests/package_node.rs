use crate::Parse;
use danube_ast::{AttributeNode, IdentNode, PackageNode, PathNode};
use danube_lex::Lex;
use danube_token::{Symbol, Token};

#[test]
fn attribute() {
    let source = "#![hello]";
    let tokens: Vec<Token> = Lex::new(source).filter_map(|token| token.ok()).collect();

    assert_eq!(
        Parse::new(tokens.as_slice()).parse_package_node(),
        Ok(PackageNode {
            attributes: vec![AttributeNode {
                path: PathNode {
                    segments: vec![IdentNode {
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
            attributes: vec![
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
            ],
            items: vec![],
        }),
    );
}
