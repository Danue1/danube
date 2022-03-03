use crate::{Context, Error, Parse, ParseList};
use danube_ast::{AttributeNode, ExpressionNode, IdentNode, PathNode, DUMMY_ATTRIBUTE_ID};

pub(crate) struct PackageAttributeNode;

impl ParseList for PackageAttributeNode {
    type Output = AttributeNode;

    fn parse_list(context: &mut Context) -> Result<Vec<Self::Output>, Error> {
        let mut attributes = vec![];

        while let Some(attribute) = PackageAttributeNode::parse(context)? {
            attributes.push(attribute);
        }

        Ok(attributes)
    }
}

impl Parse for PackageAttributeNode {
    type Output = Option<AttributeNode>;

    fn parse(context: &mut Context) -> Result<Self::Output, Error> {
        if symbol!(context.cursor => Hash) {
            if symbol!(context.cursor => Exclamation) {
                Ok(Some(AttributeNode::parse(context)?))
            } else {
                Err(Error::Invalid)
            }
        } else {
            Ok(None)
        }
    }
}

pub(crate) struct ItemAttributeNode;

impl ParseList for ItemAttributeNode {
    type Output = AttributeNode;

    fn parse_list(context: &mut Context) -> Result<Vec<Self::Output>, Error> {
        let mut attributes = vec![];

        while let Some(attribute) = ItemAttributeNode::parse(context)? {
            attributes.push(attribute);
        }

        Ok(attributes)
    }
}

impl Parse for ItemAttributeNode {
    type Output = Option<AttributeNode>;

    fn parse(context: &mut Context) -> Result<Self::Output, Error> {
        if symbol!(context.cursor => Hash) {
            Ok(Some(AttributeNode::parse(context)?))
        } else {
            Ok(None)
        }
    }
}

impl Parse for AttributeNode {
    type Output = AttributeNode;

    fn parse(context: &mut Context) -> Result<Self::Output, Error> {
        if !symbol!(context.cursor => LeftBracket) {
            return Err(Error::Invalid);
        }

        let path = if let Some(path) = PathNode::parse(context)? {
            path
        } else {
            return Err(Error::Invalid);
        };
        let args = if !symbol!(context.cursor => LeftParens) {
            vec![]
        } else {
            let mut args = vec![];

            while !symbol!(context.cursor => RightParens) {
                let ident = IdentNode::parse(context)?;
                let expression = if symbol!(context.cursor => Eq) {
                    Some(ExpressionNode::parse(context)?)
                } else {
                    None
                };

                args.push((ident, expression));

                if !symbol!(context.cursor => Comma) {
                    if symbol!(context.cursor => RightParens) {
                        break;
                    }
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
            Err(Error::Invalid)
        }
    }
}
