use crate::{Error, Parse};
use danube_ast::AttributeNode;
use std::collections::HashMap;

impl<'parse> Parse<'parse> {
    pub fn parse_package_attributes(&mut self) -> Result<Vec<AttributeNode>, Error> {
        let mut attributes = vec![];
        while let Some(attribute) = self.parse_package_attribute()? {
            attributes.push(attribute);
        }

        Ok(attributes)
    }

    pub fn parse_item_attributes(&mut self) -> Result<Vec<AttributeNode>, Error> {
        let mut attributes = vec![];
        while let Some(attribute) = self.parse_item_attribute()? {
            attributes.push(attribute);
        }

        Ok(attributes)
    }

    fn parse_package_attribute(&mut self) -> Result<Option<AttributeNode>, Error> {
        if symbol!(self.cursor => Hash) {
            if symbol!(self.cursor => Exclamation) {
                let attribute = self.parse_attribute_inner()?;

                Ok(Some(attribute))
            } else {
                Err(Error::Invalid)
            }
        } else {
            Ok(None)
        }
    }

    fn parse_item_attribute(&mut self) -> Result<Option<AttributeNode>, Error> {
        if symbol!(self.cursor => Hash) {
            let attribute = self.parse_attribute_inner()?;

            Ok(Some(attribute))
        } else {
            Ok(None)
        }
    }

    fn parse_attribute_inner(&mut self) -> Result<AttributeNode, Error> {
        if !symbol!(self.cursor => LeftBracket) {
            return Err(Error::Invalid);
        }
        let path = self.parse_path_node()?;
        let args = if symbol!(self.cursor => LeftParens) {
            std::todo!();
        } else {
            HashMap::new()
        };
        if !symbol!(self.cursor => RightBracket) {
            return Err(Error::Invalid);
        }

        Ok(AttributeNode { path, args })
    }
}

#[cfg(test)]
mod tests {
    use crate::Parse;
    use danube_ast::{AttributeNode, IdentNode, PathKind, PathNode};
    use danube_lex::Lex;
    use danube_token::Token;
    use std::collections::HashMap;

    #[test]
    fn package_attribute() {
        let source = "#![hello]";
        let tokens: Vec<Token> = Lex::new(source).filter_map(|token| token.ok()).collect();

        assert_eq!(
            Parse::new(tokens.as_slice()).parse_package_attribute(),
            Ok(Some(AttributeNode {
                path: PathNode {
                    kinds: vec![PathKind::Ident(IdentNode {
                        raw: "hello".to_string()
                    })]
                },
                args: HashMap::new(),
            }))
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
            ])
        );
    }

    #[test]
    fn item_attribute() {
        let source = "#[hello]";
        let tokens: Vec<Token> = Lex::new(source).filter_map(|token| token.ok()).collect();

        assert_eq!(
            Parse::new(tokens.as_slice()).parse_item_attribute(),
            Ok(Some(AttributeNode {
                path: PathNode {
                    kinds: vec![PathKind::Ident(IdentNode {
                        raw: "hello".to_string()
                    })]
                },
                args: HashMap::new(),
            }))
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
            ])
        );
    }
}
