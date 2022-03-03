use super::attribute_node::ItemAttributeNode;
use crate::{Context, Error, Parse, ParseList};
use danube_ast::{
    ConstantNode, FunctionNode, ImplementItemKind, ImplementItemNode, TypeAliasNode, DUMMY_NODE_ID,
};
use danube_token::keywords;

impl ParseList for ImplementItemNode {
    type Output = ImplementItemNode;

    fn parse_list(context: &mut Context) -> Result<Vec<Self::Output>, Error> {
        if !symbol!(context.cursor => LeftBrace) {
            return Err(Error::Invalid);
        }

        let mut items = vec![];

        while !symbol!(context.cursor => RightBrace) {
            items.push(ImplementItemNode::parse(context)?);
        }

        Ok(items)
    }
}

impl Parse for ImplementItemNode {
    type Output = ImplementItemNode;

    fn parse(context: &mut Context) -> Result<Self::Output, Error> {
        let attributes = ItemAttributeNode::parse_list(context)?;

        match identifier!(context.cursor) {
            Some(keywords::Type) => {
                context.cursor.next();

                Ok(ImplementItemNode {
                    id: DUMMY_NODE_ID,
                    attributes,
                    kind: ImplementItemKind::Type(TypeAliasNode::parse(context)?),
                })
            }
            Some(keywords::Const) => {
                context.cursor.next();

                Ok(ImplementItemNode {
                    id: DUMMY_NODE_ID,
                    attributes,
                    kind: ImplementItemKind::Constant(ConstantNode::parse(context)?),
                })
            }
            Some(keywords::Fn) => {
                context.cursor.next();

                Ok(ImplementItemNode {
                    id: DUMMY_NODE_ID,
                    attributes,
                    kind: ImplementItemKind::Function(FunctionNode::parse(context)?),
                })
            }
            _ => Err(Error::Invalid),
        }
    }
}
