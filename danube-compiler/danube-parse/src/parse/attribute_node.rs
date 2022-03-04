use crate::{Context, Parse};
use danube_ast::{
    AttributeArgumentNode, AttributeNode, ExpressionNode, IdentNode, PathNode, DUMMY_ATTRIBUTE_ID,
};
use danube_diagnostics::MessageBuilder;

pub(crate) struct PackageAttributeNodeList;

impl Parse for PackageAttributeNodeList {
    type Output = Vec<AttributeNode>;

    fn parse(context: &mut Context) -> Result<Self::Output, ()> {
        let mut attributes = vec![];

        while let Some(attribute) = PackageAttributeNode::parse(context)? {
            attributes.push(attribute);
        }

        Ok(attributes)
    }
}

pub(crate) struct PackageAttributeNode;

impl Parse for PackageAttributeNode {
    type Output = Option<AttributeNode>;

    fn parse(context: &mut Context) -> Result<Self::Output, ()> {
        if symbol!(context.cursor => Hash) {
            if symbol!(context.cursor => Exclamation) {
                Ok(Some(AttributeNode::parse(context)?))
            } else {
                context.report(MessageBuilder::error("Expected `!` after `#`").build())
            }
        } else {
            Ok(None)
        }
    }
}

pub(crate) struct ItemAttributeNodeList;

impl Parse for ItemAttributeNodeList {
    type Output = Vec<AttributeNode>;

    fn parse(context: &mut Context) -> Result<Self::Output, ()> {
        let mut attributes = vec![];

        while let Some(attribute) = ItemAttributeNode::parse(context)? {
            attributes.push(attribute);
        }

        Ok(attributes)
    }
}

pub(crate) struct ItemAttributeNode;

impl Parse for ItemAttributeNode {
    type Output = Option<AttributeNode>;

    fn parse(context: &mut Context) -> Result<Self::Output, ()> {
        if symbol!(context.cursor => Hash) {
            Ok(Some(AttributeNode::parse(context)?))
        } else {
            Ok(None)
        }
    }
}

impl Parse for AttributeNode {
    type Output = AttributeNode;

    fn parse(context: &mut Context) -> Result<Self::Output, ()> {
        if !symbol!(context.cursor => LeftBracket) {
            return context.report(MessageBuilder::error("Expected `[` after `#`").build());
        }

        let path = if let Some(path) = PathNode::parse(context)? {
            path
        } else {
            return context.report(MessageBuilder::error("Expected path after `[`").build());
        };
        let args = if !symbol!(context.cursor => LeftParens) {
            vec![]
        } else {
            let mut args = vec![];

            while !symbol!(context.cursor => RightParens) {
                let ident = IdentNode::parse(context)?;
                let value = if symbol!(context.cursor => Eq) {
                    Some(ExpressionNode::parse(context)?)
                } else {
                    None
                };

                args.push(AttributeArgumentNode { ident, value });

                if !symbol!(context.cursor => Comma) && symbol!(context.cursor => RightParens) {
                    break;
                }
            }

            args
        };

        if symbol!(context.cursor => RightBracket) {
            Ok(AttributeNode {
                id: DUMMY_ATTRIBUTE_ID,
                path,
                args,
            })
        } else {
            context.report(MessageBuilder::error("Expected `]` after `[`").build())
        }
    }
}
