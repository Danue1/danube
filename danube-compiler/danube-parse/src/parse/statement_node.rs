use crate::{Error, Parse};
use danube_ast::{AssignKind, AssignNode, LetNode, StatementKind, StatementNode, DUMMY_NODE_ID};
use danube_token::{keywords, TokenKind};

impl<'parse> Parse<'parse> {
    pub fn parse_statement_node(&mut self) -> Result<StatementNode, Error> {
        let kind = self.parse_statement_kind()?;

        Ok(StatementNode {
            id: DUMMY_NODE_ID,
            kind,
        })
    }

    fn parse_statement_kind(&mut self) -> Result<StatementKind, Error> {
        match self.cursor.peek().kind {
            TokenKind::Semicolon => {
                self.cursor.next();

                Ok(StatementKind::Semicolon)
            }
            TokenKind::Identifier(keywords::Break) => {
                self.cursor.next();

                Ok(StatementKind::Break)
            }
            TokenKind::Identifier(keywords::Continue) => {
                self.cursor.next();

                Ok(StatementKind::Continue)
            }
            TokenKind::Identifier(keywords::Return) => {
                self.cursor.next();

                if symbol!(self.cursor => Semicolon) {
                    Ok(StatementKind::Return(None))
                } else {
                    let expression = self.parse_expression_node()?;
                    if symbol!(self.cursor => Semicolon) {
                        Ok(StatementKind::Return(Some(expression)))
                    } else {
                        Err(Error::Invalid)
                    }
                }
            }
            TokenKind::Identifier(keywords::Let) => {
                self.cursor.next();

                let immutability = self.parse_immutability_kind()?;
                let pattern = self.parse_pattern_node()?;
                let ty = if symbol!(self.cursor => Colon) {
                    Some(self.parse_type_node()?)
                } else {
                    None
                };
                let value = if symbol!(self.cursor => Eq) {
                    Some(self.parse_expression_node()?)
                } else {
                    None
                };
                if symbol!(self.cursor => Semicolon) {
                    Ok(StatementKind::Let(Box::new(LetNode {
                        id: DUMMY_NODE_ID,
                        immutability,
                        pattern,
                        ty,
                        value,
                    })))
                } else {
                    Err(Error::Invalid)
                }
            }
            _ => {
                let expression = self.parse_expression_node()?;

                macro_rules! assign_kind {
                    ($($token:ident => $assign:ident,)+) => {
                        match self.cursor.peek().kind {
                            $(TokenKind::$token => Some(AssignKind::$assign),)+
                            _ => None
                        }
                    };
                }

                if let Some(kind) = assign_kind! {
                    Eq => Assign,

                    PlusEq => Add,
                    HyphenEq => Sub,
                    AsteriskAsteriskEq => Exp,
                    AsteriskEq => Mul,
                    SlashEq => Div,
                    PercentEq => Mod,
                    AmpersandAmpersandEq => And,
                    PipelinePipelineEq => Or,

                    AmpersandEq => BitAnd,
                    PipelineEq => BitOr,
                    CaretEq => BitXor,
                    LeftChevronLeftChevronEq => BitLeft,
                    RightChevronRightChevronEq => BitRight,
                } {
                    self.cursor.next();

                    let lhs = expression;
                    let rhs = self.parse_expression_node()?;

                    if symbol!(self.cursor => Semicolon) {
                        Ok(StatementKind::Assign(Box::new(AssignNode {
                            kind,
                            lhs,
                            rhs,
                        })))
                    } else {
                        Err(Error::Invalid)
                    }
                } else {
                    Ok(StatementKind::Expression(expression))
                }
            }
        }
    }
}
