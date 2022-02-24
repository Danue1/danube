use crate::{Error, Parse};
use danube_ast::{
    ArgumentNode, BinaryExpressionNode, BinaryOperatorKind, BlockNode, ClosureNode,
    ConditionBranch, ConditionNode, ExpressionKind, FieldNode, ForNode, FunctionCallNode,
    IdentNode, IndexNode, LoopNode, MatchBranch, MatchNode, MethodCallNode, TupleNode, WhileNode,
};
use danube_token::{keywords, Token, TokenKind};

impl<'parse> Parse<'parse> {
    pub fn parse_expression_kind(&mut self) -> Result<ExpressionKind, Error> {
        let expression = self.parse_prefix_expression_kind()?;

        self.parse_binary_expression_kind(expression)
    }

    fn parse_prefix_expression_kind(&mut self) -> Result<ExpressionKind, Error> {
        match symbol!(self.cursor) {
            Some(TokenKind::Plus) => {
                self.cursor.next();

                self.parse_prefix_expression_kind()
            }
            Some(TokenKind::Hyphen) => {
                self.cursor.next();

                Ok(ExpressionKind::Negate(Box::new(
                    self.parse_prefix_expression_kind()?,
                )))
            }
            Some(TokenKind::Exclamation) => {
                self.cursor.next();

                Ok(ExpressionKind::Not(Box::new(
                    self.parse_prefix_expression_kind()?,
                )))
            }
            Some(TokenKind::Tilde) => {
                self.cursor.next();

                Ok(ExpressionKind::BitNot(Box::new(
                    self.parse_prefix_expression_kind()?,
                )))
            }
            _ => self.parse_atomic_expression_kind(),
        }
    }

    fn parse_atomic_expression_kind(&mut self) -> Result<ExpressionKind, Error> {
        match &self.cursor.peek().kind {
            TokenKind::Literal(symbol, kind) => {
                let symbol = symbol.clone();
                let kind = kind.clone();

                self.cursor.next();

                self.parse_postfix_expression_kind(ExpressionKind::Literal(symbol, kind))
            }
            // If
            TokenKind::Identifier(keywords::If) => {
                self.cursor.next();

                macro_rules! branch {
                    () => {
                        ConditionBranch {
                            expression: if identifier!(self.cursor => Let) {
                                let pattern = self.parse_pattern_node()?;
                                if !symbol!(self.cursor => Eq) {
                                    return Err(Error::Invalid);
                                }
                                let expression = self.parse_expression_kind()?;
                                Box::new(ExpressionKind::Let(pattern, Box::new(expression)))
                            } else {
                                Box::new(self.parse_expression_kind()?)
                            },
                            block: self.parse_block_node()?,
                        }
                    };
                }

                let mut branches = vec![branch!()];
                let mut other = None;
                while identifier!(self.cursor => Else) {
                    if !identifier!(self.cursor => If) {
                        other = Some(self.parse_block_node()?);
                        break;
                    }

                    branches.push(branch!());
                }

                self.parse_postfix_expression_kind(ExpressionKind::Conditional(ConditionNode {
                    branches,
                    other,
                }))
            }
            // Loop
            TokenKind::Identifier(keywords::Loop) => {
                self.cursor.next();

                let block = self.parse_block_node()?;

                self.parse_postfix_expression_kind(ExpressionKind::Loop(LoopNode { block }))
            }
            // While
            TokenKind::Identifier(keywords::While) => {
                self.cursor.next();

                let branch = ConditionBranch {
                    expression: Box::new(self.parse_expression_kind()?),
                    block: self.parse_block_node()?,
                };

                self.parse_postfix_expression_kind(ExpressionKind::While(WhileNode { branch }))
            }
            // For
            TokenKind::Identifier(keywords::For) => {
                self.cursor.next();

                let pattern = self.parse_pattern_node()?;
                let iter = if identifier!(self.cursor => In) {
                    Box::new(self.parse_expression_kind()?)
                } else {
                    return Err(Error::Invalid);
                };
                let block = self.parse_block_node()?;

                self.parse_postfix_expression_kind(ExpressionKind::For(ForNode {
                    pattern,
                    iter,
                    block,
                }))
            }
            // Match
            TokenKind::Identifier(keywords::Match) => {
                self.cursor.next();

                let expression = self.parse_expression_kind()?;
                if !symbol!(self.cursor => LeftBrace) {
                    return Err(Error::Invalid);
                }

                let mut branches = vec![];
                while !symbol!(self.cursor => RightBrace) {
                    let pattern = self.parse_pattern_node()?;
                    if !symbol!(self.cursor => Eq) || !symbol!(self.cursor => RightChevron) {
                        return Err(Error::Invalid);
                    }

                    let block = self.parse_block_node()?;
                    branches.push(MatchBranch { pattern, block });
                }

                self.parse_postfix_expression_kind(ExpressionKind::Match(MatchNode {
                    expression: Box::new(expression),
                    branches,
                }))
            }
            // Path or Function Call
            TokenKind::Identifier(_) => {
                let path = if let Some(path) = self.parse_path_node()? {
                    ExpressionKind::Path(path)
                } else {
                    return Err(Error::Invalid);
                };
                let expression = if symbol!(self.cursor => LeftParens) {
                    ExpressionKind::FunctionCall(FunctionCallNode {
                        expression: Box::new(path),
                        arguments: self.parse_argument_nodes()?,
                    })
                } else {
                    path
                };

                self.parse_postfix_expression_kind(expression)
            }
            // Closure
            // |a| { ... }
            TokenKind::Pipeline => {
                self.cursor.next();

                let mut parameters = vec![];

                while !symbol!(self.cursor => Pipeline) {
                    let ident = self.parse_ident_node()?;
                    let ty = if symbol!(self.cursor => Colon) {
                        Some(self.parse_type_node()?)
                    } else {
                        None
                    };
                    parameters.push((ident, ty));

                    if !symbol!(self.cursor => Comma) {
                        break;
                    }
                }

                let return_type = if symbol!(self.cursor => Hyphen) {
                    if symbol!(self.cursor => RightChevron) {
                        Some(self.parse_type_node()?)
                    } else {
                        return Err(Error::Invalid);
                    }
                } else {
                    None
                };

                let block = self.parse_block_node()?;

                self.parse_postfix_expression_kind(ExpressionKind::Closure(ClosureNode {
                    parameters,
                    return_type,
                    block,
                }))
            }
            // Closure
            // || { ... }
            TokenKind::PipelinePipeline => {
                let return_type = if symbol!(self.cursor => Hyphen) {
                    if symbol!(self.cursor => RightChevron) {
                        Some(self.parse_type_node()?)
                    } else {
                        return Err(Error::Invalid);
                    }
                } else {
                    None
                };

                let block = self.parse_block_node()?;

                self.parse_postfix_expression_kind(ExpressionKind::Closure(ClosureNode {
                    parameters: vec![],
                    return_type,
                    block,
                }))
            }
            TokenKind::LeftBrace => {
                self.cursor.next();

                let mut statements = vec![];

                while !symbol!(self.cursor => RightBrace) {
                    statements.push(self.parse_statement_node()?);
                }

                self.parse_postfix_expression_kind(ExpressionKind::Block(BlockNode { statements }))
            }
            TokenKind::LeftParens => {
                self.cursor.next();

                let mut arguments = vec![];

                while !symbol!(self.cursor => RightParens) {
                    arguments.push(self.parse_prefix_expression_kind()?);
                    if !symbol!(self.cursor => Comma) {
                        break;
                    }
                }

                self.parse_postfix_expression_kind(ExpressionKind::Tuple(TupleNode { arguments }))
            }
            TokenKind::LeftBracket => {
                self.cursor.next();

                let mut expressions = vec![];

                while !symbol!(self.cursor => RightBracket) {
                    expressions.push(self.parse_prefix_expression_kind()?);
                    if !symbol!(self.cursor => Comma) {
                        break;
                    }
                }

                self.parse_postfix_expression_kind(ExpressionKind::Array(expressions))
            }
            _ => Err(Error::Invalid),
        }
    }

    fn parse_postfix_expression_kind(
        &mut self,
        expression: ExpressionKind,
    ) -> Result<ExpressionKind, Error> {
        match self.cursor.peek().kind {
            // foo?
            TokenKind::Question => {
                self.cursor.next();

                self.parse_postfix_expression_kind(ExpressionKind::Try(Box::new(expression)))
            }
            // foo()
            TokenKind::LeftParens => {
                self.cursor.next();

                let arguments = self.parse_argument_nodes()?;

                self.parse_postfix_expression_kind(ExpressionKind::FunctionCall(FunctionCallNode {
                    expression: Box::new(expression),
                    arguments,
                }))
            }
            // foo.await
            // foo.field
            // foo.method_call()
            TokenKind::Dot => {
                self.cursor.next();

                let expression = if identifier!(self.cursor => Await) {
                    ExpressionKind::Await(Box::new(expression))
                } else {
                    let ident = self.parse_ident_node()?;

                    if symbol!(self.cursor => LeftParens) {
                        let arguments = self.parse_argument_nodes()?;

                        ExpressionKind::MethodCall(MethodCallNode { ident, arguments })
                    } else {
                        ExpressionKind::Field(FieldNode {
                            expression: Box::new(expression),
                            field: ident,
                        })
                    }
                };

                self.parse_postfix_expression_kind(expression)
            }
            // foo[bar]
            TokenKind::LeftBracket => {
                self.cursor.next();

                let index = self.parse_expression_kind()?;

                if symbol!(self.cursor => RightBracket) {
                    self.parse_postfix_expression_kind(ExpressionKind::Index(IndexNode {
                        expression: Box::new(expression),
                        index: Box::new(index),
                    }))
                } else {
                    Err(Error::Invalid)
                }
            }
            _ => Ok(expression),
        }
    }

    fn parse_binary_expression_kind(
        &mut self,
        lhs: ExpressionKind,
    ) -> Result<ExpressionKind, Error> {
        macro_rules! match_operator {
            ($($kind:ident => $operator:ident,)+ _ => return Ok(lhs),) => {
                match self.cursor.peek().kind {
                    $(
                        TokenKind::$kind => {
                            self.cursor.next();
                            BinaryOperatorKind::$operator
                        }
                    )+
                    _ => return Ok(lhs),
                }
            };
        }

        let kind = match_operator! {
            Plus => Add,
            Hyphen => Sub,
            Asterisk => Mul,
            AsteriskAsterisk => Exp,
            Slash => Div,
            Percent => Mod,

            Ampersand => BitAnd,
            Pipeline => BitOr,
            Caret => BitXor,
            LeftChevronLeftChevron => BitLeft,
            RightChevronRightChevron => BitRight,

            EqEq => Equal,
            ExclamationEq => NotEqual,
            RightChevron => GreaterThan,
            LeftChevron => LessThan,
            RightChevronEq => GreaterThanOrEqual,
            LeftChevronEq => LessThanOrEqual,

            AmpersandAmpersand => And,
            PipelinePipeline => Or,

            _ => return Ok(lhs),
        };
        let rhs = self.parse_prefix_expression_kind()?;
        let expression = ExpressionKind::Binary(BinaryExpressionNode {
            kind,
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        });

        Ok(self.parse_binary_expression_kind(expression)?)
    }

    fn parse_argument_nodes(&mut self) -> Result<Vec<ArgumentNode>, Error> {
        let mut arguments = vec![];

        while !symbol!(self.cursor => RightParens) {
            arguments.push(ArgumentNode {
                ident: {
                    let mut cursor = self.cursor.clone();

                    if let Some(symbol) = identifier!(cursor) {
                        let symbol = symbol.clone();

                        if symbol!(cursor => Colon) {
                            self.cursor.next();
                            self.cursor.next();

                            Some(IdentNode { symbol })
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                },
                expression: self.parse_expression_kind()?,
            });

            if !symbol!(self.cursor => Comma) {
                break;
            }
        }

        Ok(arguments)
    }
}
