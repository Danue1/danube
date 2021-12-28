use crate::{Error, Parse};
use danube_ast::{StatementKind, StatementNode};
use danube_token::{keywords, TokenKind};

impl<'parse> Parse<'parse> {
    pub fn parse_statement_node(&mut self) -> Result<StatementNode, Error> {
        let kind = self.parse_statement_kind()?;

        Ok(StatementNode {
            id: self.resolver.next_id().into(),
            kind,
        })
    }

    fn parse_statement_kind(&mut self) -> Result<StatementKind, Error> {
        match self.cursor.next() {
            Some(token) => match token.kind {
                TokenKind::Semicolon => Ok(StatementKind::Semicolon),
                TokenKind::Identifier(keywords::Break) => Ok(StatementKind::Break),
                TokenKind::Identifier(keywords::Continue) => Ok(StatementKind::Continue),
                TokenKind::Identifier(keywords::Return) => {
                    if symbol!(self.cursor => Semicolon) {
                        Ok(StatementKind::Return(None))
                    } else {
                        let expression = self.parse_expression_kind()?;
                        if symbol!(self.cursor => Semicolon) {
                            Ok(StatementKind::Return(Some(expression)))
                        } else {
                            Err(Error::Invalid)
                        }
                    }
                }
                _ => std::todo!(),
            },
            _ => std::todo!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Parse;
    use danube_ast::{
        ExpressionKind, Id, IdentNode, PathKind, PathNode, StatementId, StatementKind,
        StatementNode,
    };
    use danube_lex::Lex;
    use danube_token::{SymbolInterner, Token};

    #[test]
    fn statement_semicolon() {
        let source = ";";
        let tokens: Vec<Token> = Lex::new(source).filter_map(|token| token.ok()).collect();

        assert_eq!(
            Parse::new(tokens.as_slice()).parse_statement_node(),
            Ok(StatementNode {
                id: StatementId(Id(0)),
                kind: StatementKind::Semicolon
            })
        );
    }

    #[test]
    fn statement_break() {
        let source = "break";
        let tokens: Vec<Token> = Lex::new(source).filter_map(|token| token.ok()).collect();

        assert_eq!(
            Parse::new(tokens.as_slice()).parse_statement_node(),
            Ok(StatementNode {
                id: StatementId(Id(0)),
                kind: StatementKind::Break
            })
        );
    }

    #[test]
    fn statement_continue() {
        let source = "continue";
        let tokens: Vec<Token> = Lex::new(source).filter_map(|token| token.ok()).collect();

        assert_eq!(
            Parse::new(tokens.as_slice()).parse_statement_node(),
            Ok(StatementNode {
                id: StatementId(Id(0)),
                kind: StatementKind::Continue
            })
        );
    }

    #[test]
    fn statement_return_without_expression() {
        let source = "return;";
        let tokens: Vec<Token> = Lex::new(source).filter_map(|token| token.ok()).collect();

        assert_eq!(
            Parse::new(tokens.as_slice()).parse_statement_node(),
            Ok(StatementNode {
                id: StatementId(Id(0)),
                kind: StatementKind::Return(None)
            })
        );
    }

    #[test]
    fn statement_return_with_expression() {
        let source = "return hello;";
        let lexer = Lex::new(source);
        let tokens: Vec<Token> = lexer.filter_map(|token| token.ok()).collect();
        let mut interner = SymbolInterner::default();

        assert_eq!(
            Parse::new(tokens.as_slice()).parse_statement_node(),
            Ok(StatementNode {
                id: StatementId(Id(0)),
                kind: StatementKind::Return(Some(ExpressionKind::Path(PathNode {
                    kinds: vec![PathKind::Ident(IdentNode {
                        symbol: interner.intern("hello")
                    })]
                })))
            })
        );
    }
}
