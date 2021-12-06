use crate::{Error, Parse};
use danube_ast::{IdentNode, PathKind, PathNode};
use danube_token::{Keyword, Token, TokenKind};

impl<'parse> Parse<'parse> {
    pub fn parse_path_node(&mut self) -> Result<PathNode, Error> {
        let mut kinds = vec![];
        loop {
            kinds.push(self.parse_path_kind()?);

            if !symbol!(self.cursor => ColonColon) {
                break;
            }
        }

        Ok(PathNode { kinds })
    }

    fn parse_path_kind(&mut self) -> Result<PathKind, Error> {
        match self.cursor.peek() {
            Some(Token {
                span: _,
                kind: TokenKind::Keyword(Keyword::Package),
            }) => {
                self.cursor.next();
                Ok(PathKind::Package)
            }
            Some(Token {
                span: _,
                kind: TokenKind::Keyword(Keyword::Super),
            }) => {
                self.cursor.next();
                Ok(PathKind::Super)
            }
            Some(Token {
                span: _,
                kind: TokenKind::Identifier(identifier),
            }) => {
                let raw = identifier.value.to_string();

                self.cursor.next();

                Ok(PathKind::Ident(IdentNode { raw }))
            }
            _ => Err(Error::Invalid),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Parse;
    use danube_ast::{IdentNode, PathKind, PathNode};
    use danube_lex::Lex;
    use danube_token::Token;

    #[test]
    fn _super() {
        let source = "super";
        let tokens: Vec<Token> = Lex::new(source).filter_map(|token| token.ok()).collect();

        assert_eq!(
            Parse::new(tokens.as_slice()).parse_path_node(),
            Ok(PathNode {
                kinds: vec![PathKind::Super]
            })
        );
    }

    #[test]
    fn package() {
        let source = "package";
        let tokens: Vec<Token> = Lex::new(source).filter_map(|token| token.ok()).collect();

        assert_eq!(
            Parse::new(tokens.as_slice()).parse_path_node(),
            Ok(PathNode {
                kinds: vec![PathKind::Package]
            })
        );
    }

    #[test]
    fn super_with_ident() {
        let source = "super::hello";
        let tokens: Vec<Token> = Lex::new(source).filter_map(|token| token.ok()).collect();

        assert_eq!(
            Parse::new(tokens.as_slice()).parse_path_node(),
            Ok(PathNode {
                kinds: vec![
                    PathKind::Super,
                    PathKind::Ident(IdentNode {
                        raw: "hello".to_string()
                    })
                ]
            })
        );
    }

    #[test]
    fn package_with_ident() {
        let source = "package::hello";
        let tokens: Vec<Token> = Lex::new(source).filter_map(|token| token.ok()).collect();

        assert_eq!(
            Parse::new(tokens.as_slice()).parse_path_node(),
            Ok(PathNode {
                kinds: vec![
                    PathKind::Package,
                    PathKind::Ident(IdentNode {
                        raw: "hello".to_string()
                    })
                ]
            })
        );
    }

    #[test]
    fn ident() {
        let source = "hello";
        let tokens: Vec<Token> = Lex::new(source).filter_map(|token| token.ok()).collect();

        assert_eq!(
            Parse::new(tokens.as_slice()).parse_path_node(),
            Ok(PathNode {
                kinds: vec![PathKind::Ident(IdentNode {
                    raw: "hello".to_string()
                }),]
            })
        );
    }

    #[test]
    fn idents() {
        let source = "hello::world";
        let tokens: Vec<Token> = Lex::new(source).filter_map(|token| token.ok()).collect();

        assert_eq!(
            Parse::new(tokens.as_slice()).parse_path_node(),
            Ok(PathNode {
                kinds: vec![
                    PathKind::Ident(IdentNode {
                        raw: "hello".to_string()
                    }),
                    PathKind::Ident(IdentNode {
                        raw: "world".to_string()
                    }),
                ]
            })
        );
    }
}
