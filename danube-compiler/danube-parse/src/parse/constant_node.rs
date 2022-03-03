use crate::{Context, Error, Parse};
use danube_ast::{ConstantNode, ExpressionNode, PatternNode, TypeNode};

impl Parse for ConstantNode {
    type Output = ConstantNode;

    fn parse(context: &mut Context) -> Result<Self::Output, Error> {
        let pattern = PatternNode::parse(context)?;
        let ty = if symbol!(context.cursor => Colon) {
            TypeNode::parse(context)?
        } else {
            return Err(Error::Invalid);
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
            Err(Error::Invalid)
        }
    }
}
