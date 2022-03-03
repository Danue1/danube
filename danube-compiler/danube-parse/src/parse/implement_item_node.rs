use super::attribute_node::ItemAttributeNodeList;
use crate::{Context, Parse};
use danube_ast::{
    ConstantNode, FunctionNode, ImplementItemKind, ImplementItemNode, TypeAliasNode, DUMMY_NODE_ID,
};
use danube_diagnostics::MessageBuilder;
use danube_token::keywords;

pub(crate) struct ImplementItemNodeList;

impl Parse for ImplementItemNodeList {
    type Output = Vec<ImplementItemNode>;

    fn parse(context: &mut Context) -> Result<Self::Output, ()> {
        if !symbol!(context.cursor => LeftBrace) {
            return context.report(MessageBuilder::error("Expected `{`").build());
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

    fn parse(context: &mut Context) -> Result<Self::Output, ()> {
        let attributes = ItemAttributeNodeList::parse(context)?;

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
            _ => context.report(MessageBuilder::error("Expected `type`, `const`, or `fn`").build()),
        }
    }
}
