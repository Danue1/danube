use crate::{Context, Parse};
use danube_ast::{GenericNode, IdentNode, PathNode, DUMMY_NODE_ID};
use danube_diagnostics::MessageBuilder;

pub(crate) struct GenericNodeList;

impl Parse for GenericNodeList {
    type Output = Vec<GenericNode>;

    fn parse(context: &mut Context) -> Result<Self::Output, ()> {
        if !symbol!(context.cursor => LeftChevron) {
            return Ok(vec![]);
        }

        let mut generics = vec![];

        while !symbol!(context.cursor => RightChevron) {
            generics.push(GenericNode::parse(context)?);

            if !symbol!(context.cursor => Comma) && symbol!(context.cursor => RightChevron) {
                break;
            }
        }

        Ok(generics)
    }
}

impl Parse for GenericNode {
    type Output = GenericNode;

    fn parse(context: &mut Context) -> Result<Self::Output, ()> {
        let ident = IdentNode::parse(context)?;
        let traits = if symbol!(context.cursor => Colon) {
            let mut paths = if let Some(path) = PathNode::parse(context)? {
                vec![path]
            } else {
                return context.report(MessageBuilder::error("Expected trait path").build());
            };

            while symbol!(context.cursor => Plus) {
                if let Some(path) = PathNode::parse(context)? {
                    paths.push(path);
                } else {
                    return context.report(MessageBuilder::error("Expected trait path").build());
                }
            }

            paths
        } else {
            vec![]
        };
        let default = if symbol!(context.cursor => Eq) {
            if let Some(path) = PathNode::parse(context)? {
                Some(path)
            } else {
                return context.report(MessageBuilder::error("Expected default type").build());
            }
        } else {
            None
        };

        Ok(GenericNode {
            id: DUMMY_NODE_ID,
            ident,
            traits,
            default,
        })
    }
}
