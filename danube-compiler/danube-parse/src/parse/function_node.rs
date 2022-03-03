use super::function_parameter_node::FunctionParameterNodeList;
use crate::{Context, Error, Parse, ParseList};
use danube_ast::{BlockNode, FunctionNode, GenericNode, IdentNode, TypeNode};

impl Parse for FunctionNode {
    type Output = FunctionNode;

    fn parse(context: &mut Context) -> Result<Self::Output, Error> {
        let ident = IdentNode::parse(context)?;
        let generics = GenericNode::parse_list(context)?;
        let (self_type, parameters) = FunctionParameterNodeList::parse(context)?;
        let return_type = FunctionReturnType::parse(context)?;
        let block = if symbol!(context.cursor => Semicolon) {
            None
        } else {
            Some(BlockNode::parse(context)?)
        };

        Ok(FunctionNode {
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

    fn parse(context: &mut Context) -> Result<Option<TypeNode>, Error> {
        if symbol!(context.cursor => HyphenRightChevron) {
            Ok(Some(TypeNode::parse(context)?))
        } else {
            Ok(None)
        }
    }
}
