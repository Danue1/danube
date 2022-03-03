use crate::{Context, Parse};
use danube_ast::{
    AssignKind, AssignNode, ExpressionNode, ImmutabilityKind, LetNode, PatternNode, StatementKind,
    StatementNode, TypeNode, DUMMY_NODE_ID,
};
use danube_diagnostics::MessageBuilder;
use danube_token::{keywords, TokenKind};

impl Parse for StatementNode {
    type Output = StatementNode;

    fn parse(context: &mut Context) -> Result<Self::Output, ()> {
        Ok(StatementNode {
            id: DUMMY_NODE_ID,
            kind: StatementKind::parse(context)?,
        })
    }
}

impl Parse for StatementKind {
    type Output = StatementKind;

    fn parse(context: &mut Context) -> Result<Self::Output, ()> {
        match context.cursor.peek().kind {
            TokenKind::Semicolon => {
                context.cursor.next();

                Ok(StatementKind::Semicolon)
            }
            TokenKind::Identifier(keywords::Break) => {
                context.cursor.next();

                Ok(StatementKind::Break)
            }
            TokenKind::Identifier(keywords::Continue) => {
                context.cursor.next();

                Ok(StatementKind::Continue)
            }
            TokenKind::Identifier(keywords::Return) => {
                context.cursor.next();

                if symbol!(context.cursor => Semicolon) {
                    Ok(StatementKind::Return(None))
                } else {
                    let expression = ExpressionNode::parse(context)?;
                    if symbol!(context.cursor => Semicolon) {
                        Ok(StatementKind::Return(Some(expression)))
                    } else {
                        context.report(MessageBuilder::error("Expected `;`").build())
                    }
                }
            }
            TokenKind::Identifier(keywords::Let) => {
                context.cursor.next();

                let immutability = ImmutabilityKind::parse(context)?;
                let pattern = PatternNode::parse(context)?;
                let ty = if symbol!(context.cursor => Colon) {
                    Some(TypeNode::parse(context)?)
                } else {
                    None
                };
                let value = if symbol!(context.cursor => Eq) {
                    Some(ExpressionNode::parse(context)?)
                } else {
                    None
                };
                if symbol!(context.cursor => Semicolon) {
                    Ok(StatementKind::Let(Box::new(LetNode {
                        id: DUMMY_NODE_ID,
                        immutability,
                        pattern,
                        ty,
                        value,
                    })))
                } else {
                    context.report(MessageBuilder::error("Expected `;`").build())
                }
            }
            _ => {
                let expression = ExpressionNode::parse(context)?;

                macro_rules! assign_kind {
                    ($($token:ident => $assign:ident,)+) => {
                        match context.cursor.peek().kind {
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
                    context.cursor.next();

                    let lhs = expression;
                    let rhs = ExpressionNode::parse(context)?;

                    if symbol!(context.cursor => Semicolon) {
                        Ok(StatementKind::Assign(Box::new(AssignNode {
                            kind,
                            lhs,
                            rhs,
                        })))
                    } else {
                        context.report(MessageBuilder::error("Expected `;`").build())
                    }
                } else {
                    Ok(StatementKind::Expression(expression))
                }
            }
        }
    }
}
