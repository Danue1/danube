use crate::{Error, Parse};
use danube_ast::PackageNode;

impl<'parse> Parse<'parse> {
    pub fn parse_package_node(&mut self) -> Result<PackageNode, Error> {
        let attributes = self.parse_package_attributes()?;
        let items = self.parse_item_nodes()?;

        Ok(PackageNode { attributes, items })
    }
}

#[cfg(test)]
mod tests {
    use crate::Parse;
    use danube_ast::{AttributeNode, IdentNode, PackageNode, PathKind, PathNode};
    use danube_lex::Lex;
    use danube_token::Token;
    use std::collections::HashMap;

    #[test]
    fn attribute() {
        let source = "#![hello]";
        let tokens: Vec<Token> = Lex::new(source).filter_map(|token| token.ok()).collect();

        assert_eq!(
            Parse::new(tokens.as_slice()).parse_package_node(),
            Ok(PackageNode {
                attributes: vec![AttributeNode {
                    path: PathNode {
                        kinds: vec![PathKind::Ident(IdentNode {
                            raw: "hello".to_string()
                        })]
                    },
                    args: HashMap::new(),
                }],
                items: vec![],
            })
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
                            kinds: vec![PathKind::Ident(IdentNode {
                                raw: "hello".to_string()
                            })]
                        },
                        args: HashMap::new(),
                    },
                    AttributeNode {
                        path: PathNode {
                            kinds: vec![PathKind::Ident(IdentNode {
                                raw: "hello".to_string()
                            })]
                        },
                        args: HashMap::new(),
                    }
                ],
                items: vec![],
            })
        );
    }
}
