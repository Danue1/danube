use super::function_parameter_node::FunctionParameterNodeList;
use super::generic_node::GenericNodeList;
use crate::{Context, Parse};
use danube_ast::{BlockNode, FunctionNode, IdentNode, TypeNode, DUMMY_NODE_ID};

impl Parse for FunctionNode {
    type Output = FunctionNode;

    fn parse(context: &mut Context) -> Result<Self::Output, ()> {
        let ident = IdentNode::parse(context)?;
        let generics = GenericNodeList::parse(context)?;
        let (self_type, parameters) = FunctionParameterNodeList::parse(context)?;
        let return_type = FunctionReturnType::parse(context)?;
        let block = if symbol!(context.cursor => Semicolon) {
            None
        } else {
            Some(BlockNode::parse(context)?)
        };

        Ok(FunctionNode {
            id: DUMMY_NODE_ID,
            ident,
            generics,
            self_type,
            parameters,
            return_type,
            block,
        })
    }
}

struct FunctionReturnType;

impl Parse for FunctionReturnType {
    type Output = Option<TypeNode>;

    fn parse(context: &mut Context) -> Result<Option<TypeNode>, ()> {
        if symbol!(context.cursor => HyphenRightChevron) {
            Ok(Some(TypeNode::parse(context)?))
        } else {
            Ok(None)
        }
    }
}
