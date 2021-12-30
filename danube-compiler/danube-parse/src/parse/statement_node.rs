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
