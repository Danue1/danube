use super::argument_node::ArgumentNodeList;
use super::expression_node::PrefixExpressionNode;
use crate::{Context, Error, Parse};
use danube_ast::{
    BinaryExpressionNode, BinaryOperatorKind, BlockNode, ClosureNode, ConditionBranch,
    ConditionNode, ExpressionKind, ExpressionNode, FieldNode, ForNode, FunctionCallNode, IdentNode,
    IndexNode, LoopNode, MatchBranch, MatchNode, MethodCallNode, PathNode, PatternNode,
    StatementNode, TupleNode, TypeNode, WhileNode, DUMMY_NODE_ID,
};
use danube_token::{keywords, TokenKind};

impl Parse for ExpressionKind {
    type Output = ExpressionKind;

    fn parse(context: &mut Context) -> Result<Self::Output, Error> {
        let expression = PrefixExpressionKind::parse(context)?;

        parse_binary_expression_kind(context, expression)
    }
}

pub(crate) struct PrefixExpressionKind;

impl Parse for PrefixExpressionKind {
    type Output = ExpressionKind;

    fn parse(context: &mut Context) -> Result<Self::Output, Error> {
        match symbol!(context.cursor) {
            Some(TokenKind::Plus) => {
                context.cursor.next();

                PrefixExpressionKind::parse(context)
            }
            Some(TokenKind::Hyphen) => {
                context.cursor.next();

                Ok(ExpressionKind::Negate(Box::new(
                    PrefixExpressionNode::parse(context)?,
                )))
            }
            Some(TokenKind::Exclamation) => {
                context.cursor.next();

                Ok(ExpressionKind::Not(Box::new(PrefixExpressionNode::parse(
                    context,
                )?)))
            }
            _ => AtomicExpressionKind::parse(context),
        }
    }
}

struct AtomicExpressionKind;

impl Parse for AtomicExpressionKind {
    type Output = ExpressionKind;

    fn parse(context: &mut Context) -> Result<Self::Output, Error> {
        match &context.cursor.peek().kind {
            TokenKind::Literal(symbol, kind) => {
                let symbol = symbol.clone();
                let kind = kind.clone();

                context.cursor.next();

                parse_postfix_expression_kind(context, ExpressionKind::Literal(symbol, kind))
            }
            // If
            TokenKind::Identifier(keywords::If) => {
                context.cursor.next();

                macro_rules! branch {
                    () => {
                        ConditionBranch {
                            expression: if identifier!(context.cursor => Let) {
                                let pattern = PatternNode::parse(context)?;
                                if !symbol!(context.cursor => Eq) {
                                    return Err(Error::Invalid);
                                }
                                let expression = ExpressionNode::parse(context)?;
                                Box::new(ExpressionNode {
                                    id: DUMMY_NODE_ID,
                                    kind: ExpressionKind::Let(pattern, Box::new(expression)),
                                })
                            } else {
                                Box::new(ExpressionNode::parse(context)?)
                            },
                            block: BlockNode::parse(context)?,
                        }
                    };
                }

                let mut branches = vec![branch!()];
                let mut other = None;
                while identifier!(context.cursor => Else) {
                    if !identifier!(context.cursor => If) {
                        other = Some(BlockNode::parse(context)?);
                        break;
                    }

                    branches.push(branch!());
                }

                parse_postfix_expression_kind(
                    context,
                    ExpressionKind::Conditional(ConditionNode { branches, other }),
                )
            }
            // Loop
            TokenKind::Identifier(keywords::Loop) => {
                context.cursor.next();

                let block = BlockNode::parse(context)?;

                parse_postfix_expression_kind(context, ExpressionKind::Loop(LoopNode { block }))
            }
            // While
            TokenKind::Identifier(keywords::While) => {
                context.cursor.next();

                let branch = ConditionBranch {
                    expression: Box::new(ExpressionNode::parse(context)?),
                    block: BlockNode::parse(context)?,
                };

                parse_postfix_expression_kind(context, ExpressionKind::While(WhileNode { branch }))
            }
            // For
            TokenKind::Identifier(keywords::For) => {
                context.cursor.next();

                let pattern = PatternNode::parse(context)?;
                let iter = if identifier!(context.cursor => In) {
                    Box::new(ExpressionNode::parse(context)?)
                } else {
                    return Err(Error::Invalid);
                };
                let block = BlockNode::parse(context)?;

                parse_postfix_expression_kind(
                    context,
                    ExpressionKind::For(ForNode {
                        pattern,
                        iter,
                        block,
                    }),
                )
            }
            // Match
            TokenKind::Identifier(keywords::Match) => {
                context.cursor.next();

                let expression = ExpressionNode::parse(context)?;
                if !symbol!(context.cursor => LeftBrace) {
                    return Err(Error::Invalid);
                }

                let mut branches = vec![];
                while !symbol!(context.cursor => RightBrace) {
                    let pattern = PatternNode::parse(context)?;
                    if !symbol!(context.cursor => EqRightChevron) {
                        return Err(Error::Invalid);
                    }

                    let block = BlockNode::parse(context)?;
                    branches.push(MatchBranch { pattern, block });

                    symbol!(context.cursor => Comma);
                }

                parse_postfix_expression_kind(
                    context,
                    ExpressionKind::Match(MatchNode {
                        expression: Box::new(expression),
                        branches,
                    }),
                )
            }
            // Path or Function Call
            TokenKind::Identifier(_) => {
                let path = if let Some(path) = PathNode::parse(context)? {
                    ExpressionKind::Path(path)
                } else {
                    return Err(Error::Invalid);
                };
                let expression = if symbol!(context.cursor => LeftParens) {
                    ExpressionKind::FunctionCall(FunctionCallNode {
                        expression: Box::new(ExpressionNode {
                            id: DUMMY_NODE_ID,
                            kind: path,
                        }),
                        arguments: ArgumentNodeList::parse(context)?,
                    })
                } else {
                    path
                };

                parse_postfix_expression_kind(context, expression)
            }
            // Closure
            // |a| { ... }
            TokenKind::Pipeline => {
                context.cursor.next();

                let mut parameters = vec![];

                while !symbol!(context.cursor => Pipeline) {
                    let ident = IdentNode::parse(context)?;
                    let ty = if symbol!(context.cursor => Colon) {
                        Some(TypeNode::parse(context)?)
                    } else {
                        None
                    };
                    parameters.push((ident, ty));

                    if !symbol!(context.cursor => Comma) {
                        break;
                    }
                }

                let return_type = if symbol!(context.cursor => HyphenRightChevron) {
                    Some(TypeNode::parse(context)?)
                } else {
                    None
                };

                let block = BlockNode::parse(context)?;

                parse_postfix_expression_kind(
                    context,
                    ExpressionKind::Closure(ClosureNode {
                        parameters,
                        return_type,
                        block,
                    }),
                )
            }
            // Closure
            // || { ... }
            TokenKind::PipelinePipeline => {
                let return_type = if symbol!(context.cursor => Hyphen) {
                    if symbol!(context.cursor => RightChevron) {
                        Some(TypeNode::parse(context)?)
                    } else {
                        return Err(Error::Invalid);
                    }
                } else {
                    None
                };

                let block = BlockNode::parse(context)?;

                parse_postfix_expression_kind(
                    context,
                    ExpressionKind::Closure(ClosureNode {
                        parameters: vec![],
                        return_type,
                        block,
                    }),
                )
            }
            TokenKind::LeftBrace => {
                context.cursor.next();

                let mut statements = vec![];

                while !symbol!(context.cursor => RightBrace) {
                    statements.push(StatementNode::parse(context)?);
                }

                parse_postfix_expression_kind(
                    context,
                    ExpressionKind::Block(BlockNode {
                        id: DUMMY_NODE_ID,
                        statements,
                    }),
                )
            }
            TokenKind::LeftParens => {
                context.cursor.next();

                let mut arguments = vec![];

                while !symbol!(context.cursor => RightParens) {
                    arguments.push(PrefixExpressionNode::parse(context)?);

                    if !symbol!(context.cursor => Comma) {
                        if symbol!(context.cursor => RightParens) {
                            break;
                        }
                    }
                }

                parse_postfix_expression_kind(
                    context,
                    ExpressionKind::Tuple(TupleNode { arguments }),
                )
            }
            TokenKind::LeftBracket => {
                context.cursor.next();

                let mut expressions = vec![];

                while !symbol!(context.cursor => RightBracket) {
                    expressions.push(PrefixExpressionNode::parse(context)?);

                    if !symbol!(context.cursor => Comma) {
                        if symbol!(context.cursor => RightBracket) {
                            break;
                        }
                    }
                }

                parse_postfix_expression_kind(context, ExpressionKind::Array(expressions))
            }
            _ => Err(Error::Invalid),
        }
    }
}

fn parse_postfix_expression_kind(
    context: &mut Context,
    expression: ExpressionKind,
) -> Result<ExpressionKind, Error> {
    match context.cursor.peek().kind {
        // foo?
        TokenKind::Question => {
            context.cursor.next();

            parse_postfix_expression_kind(
                context,
                ExpressionKind::Try(Box::new(ExpressionNode {
                    id: DUMMY_NODE_ID,
                    kind: expression,
                })),
            )
        }
        // foo()
        TokenKind::LeftParens => {
            context.cursor.next();

            let arguments = ArgumentNodeList::parse(context)?;

            parse_postfix_expression_kind(
                context,
                ExpressionKind::FunctionCall(FunctionCallNode {
                    expression: Box::new(ExpressionNode {
                        id: DUMMY_NODE_ID,
                        kind: expression,
                    }),
                    arguments,
                }),
            )
        }
        // foo.await
        // foo.field
        // foo.method_call()
        TokenKind::Dot => {
            context.cursor.next();

            let expression = if identifier!(context.cursor => Await) {
                ExpressionKind::Await(Box::new(ExpressionNode {
                    id: DUMMY_NODE_ID,
                    kind: expression,
                }))
            } else {
                let ident = IdentNode::parse(context)?;

                if symbol!(context.cursor => LeftParens) {
                    ExpressionKind::MethodCall(MethodCallNode {
                        ident,
                        arguments: ArgumentNodeList::parse(context)?,
                    })
                } else {
                    ExpressionKind::Field(FieldNode {
                        expression: Box::new(ExpressionNode {
                            id: DUMMY_NODE_ID,
                            kind: expression,
                        }),
                        field: ident,
                    })
                }
            };

            parse_postfix_expression_kind(context, expression)
        }
        // foo[bar]
        TokenKind::LeftBracket => {
            context.cursor.next();

            let index = ExpressionNode::parse(context)?;

            if symbol!(context.cursor => RightBracket) {
                parse_postfix_expression_kind(
                    context,
                    ExpressionKind::Index(IndexNode {
                        expression: Box::new(ExpressionNode {
                            id: DUMMY_NODE_ID,
                            kind: expression,
                        }),
                        index: Box::new(index),
                    }),
                )
            } else {
                Err(Error::Invalid)
            }
        }
        _ => Ok(expression),
    }
}

fn parse_binary_expression_kind(
    context: &mut Context,
    lhs: ExpressionKind,
) -> Result<ExpressionKind, Error> {
    macro_rules! match_operator {
        ($($kind:ident => $operator:ident,)+ _ => return Ok(lhs),) => {
            match context.cursor.peek().kind {
                $(
                    TokenKind::$kind => {
                        context.cursor.next();
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
    let rhs = PrefixExpressionNode::parse(context)?;
    let expression = ExpressionKind::Binary(BinaryExpressionNode {
        kind,
        lhs: Box::new(ExpressionNode {
            id: DUMMY_NODE_ID,
            kind: lhs,
        }),
        rhs: Box::new(rhs),
    });

    Ok(parse_binary_expression_kind(context, expression)?)
}
