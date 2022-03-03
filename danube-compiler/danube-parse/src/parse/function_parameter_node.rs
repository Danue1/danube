use crate::{Context, Error, Parse};
use danube_ast::{FunctionParameterNode, IdentNode, ImmutabilityKind, TypeNode, DUMMY_NODE_ID};

pub(crate) struct FunctionParameterNodeList;

impl Parse for FunctionParameterNodeList {
    type Output = (Option<ImmutabilityKind>, Vec<FunctionParameterNode>);

    fn parse(context: &mut Context) -> Result<Self::Output, Error> {
        if !symbol!(context.cursor => LeftParens) {
            return Err(Error::Invalid);
        }

        let immutability = if let Ok(immutability) = ImmutabilityKind::parse(context) {
            if identifier!(context.cursor => SelfLower) {
                Some(immutability)
            } else if immutability == ImmutabilityKind::Nope {
                return Err(Error::Invalid);
            } else {
                None
            }
        } else if identifier!(context.cursor => SelfLower) {
            Some(ImmutabilityKind::Nope)
        } else {
            None
        };

        let mut parameters = vec![];

        while !symbol!(context.cursor => RightParens) {
            parameters.push(FunctionParameterNode::parse(context)?);

            if !symbol!(context.cursor => Comma) {
                if symbol!(context.cursor => RightParens) {
                    break;
                }

                return Err(Error::Invalid);
            }
        }

        Ok((immutability, parameters))
    }
}

impl Parse for FunctionParameterNode {
    type Output = FunctionParameterNode;

    fn parse(context: &mut Context) -> Result<Self::Output, Error> {
        let argument_label = IdentNode::parse(context)?;
        let (parameter_label, ty) = if symbol!(context.cursor => Colon) {
            (None, TypeNode::parse(context)?)
        } else {
            (Some(IdentNode::parse(context)?), TypeNode::parse(context)?)
        };

        Ok(FunctionParameterNode {
            id: DUMMY_NODE_ID,
            argument_label,
            parameter_label,
            ty,
        })
    }
}
