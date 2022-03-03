use crate::{Context, Parse};
use danube_ast::{ConstantNode, ExpressionNode, PatternNode, TypeNode};
use danube_diagnostics::MessageBuilder;

impl Parse for ConstantNode {
    type Output = ConstantNode;

    fn parse(context: &mut Context) -> Result<Self::Output, ()> {
        let pattern = PatternNode::parse(context)?;
        let ty = if symbol!(context.cursor => Colon) {
            TypeNode::parse(context)?
        } else {
            return context.report(MessageBuilder::error("Expected `:`").build());
        };
        let expression = if symbol!(context.cursor => Eq) {
            Some(ExpressionNode::parse(context)?)
        } else {
            None
        };

        if symbol!(context.cursor => Semicolon) {
            Ok(ConstantNode {
                pattern,
                ty,
                expression,
            })
        } else {
            context.report(MessageBuilder::error("Expected `;`").build())
        }
    }
}
