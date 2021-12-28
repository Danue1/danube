use crate::{Error, Parse};
use danube_ast::UseNode;

impl<'parse> Parse<'parse> {
    pub fn parse_use_node(&mut self) -> Result<UseNode, Error> {
        let path = self.parse_path_node()?;
        if symbol!(self.cursor => Semicolon) {
            Ok(UseNode { path })
        } else {
            Err(Error::Invalid)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Parse;
    use danube_ast::{IdentNode, PathKind, PathNode, UseNode};
    use danube_lex::Lex;
    use danube_token::{SymbolInterner, Token};

    #[test]
    fn package() {
        let source = "package;";
        let tokens: Vec<Token> = Lex::new(source).filter_map(|token| token.ok()).collect();

        assert_eq!(
            Parse::new(tokens.as_slice()).parse_use_node(),
            Ok(UseNode {
                path: PathNode {
                    kinds: vec![PathKind::Package]
                },
            })
        );
    }

    #[test]
    fn package_with_ident() {
        let source = "package::world;";
        let tokens: Vec<Token> = Lex::new(source).filter_map(|token| token.ok()).collect();
        let mut interner = SymbolInterner::default();

        assert_eq!(
            Parse::new(tokens.as_slice()).parse_use_node(),
            Ok(UseNode {
                path: PathNode {
                    kinds: vec![
                        PathKind::Package,
                        PathKind::Ident(IdentNode {
                            symbol: interner.intern("hello")
                        })
                    ]
                },
            })
        );
    }

    #[test]
    fn _super() {
        let source = "super;";
        let tokens: Vec<Token> = Lex::new(source).filter_map(|token| token.ok()).collect();

        assert_eq!(
            Parse::new(tokens.as_slice()).parse_use_node(),
            Ok(UseNode {
                path: PathNode {
                    kinds: vec![PathKind::Super]
                },
            })
        );
    }

    #[test]
    fn super_with_ident() {
        let source = "super::world;";
        let tokens: Vec<Token> = Lex::new(source).filter_map(|token| token.ok()).collect();
        let mut interner = SymbolInterner::default();

        assert_eq!(
            Parse::new(tokens.as_slice()).parse_use_node(),
            Ok(UseNode {
                path: PathNode {
                    kinds: vec![
                        PathKind::Super,
                        PathKind::Ident(IdentNode {
                            symbol: interner.intern("world")
                        })
                    ]
                },
            })
        );
    }
}
