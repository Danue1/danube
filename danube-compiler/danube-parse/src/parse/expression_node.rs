use super::expression_kind::PrefixExpressionKind;
use crate::{Context, Parse};
use danube_ast::{ExpressionKind, ExpressionNode, DUMMY_NODE_ID};

impl Parse for ExpressionNode {
    type Output = ExpressionNode;

    fn parse(context: &mut Context) -> Result<Self::Output, ()> {
        Ok(ExpressionNode {
            id: DUMMY_NODE_ID,
            kind: ExpressionKind::parse(context)?,
        })
    }
}

pub(crate) struct PrefixExpressionNode;

impl Parse for PrefixExpressionNode {
    type Output = ExpressionNode;

    fn parse(context: &mut Context) -> Result<Self::Output, ()> {
        Ok(ExpressionNode {
            id: DUMMY_NODE_ID,
            kind: PrefixExpressionKind::parse(context)?,
        })
    }
}
