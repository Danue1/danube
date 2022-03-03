use super::attribute_node::ItemAttributeNodeList;
use crate::{Context, Error, Parse};
use danube_ast::{
    ConstantNode, EnumNode, FunctionNode, ImplementNode, ItemKind, ItemNode, TraitNode,
    TypeAliasNode, UseNode, VisibilityKind, DUMMY_NODE_ID,
};
use danube_token::keywords;

pub(crate) struct ItemNodeList;

impl Parse for ItemNodeList {
    type Output = Vec<ItemNode>;

    fn parse(context: &mut Context) -> Result<Self::Output, Error> {
        let mut items = vec![];

        while let Some(item) = ItemNode::parse(context)? {
            items.push(item);
        }

        Ok(items)
    }
}

impl Parse for ItemNode {
    type Output = Option<ItemNode>;

    fn parse(context: &mut Context) -> Result<Self::Output, Error> {
        let attributes = ItemAttributeNodeList::parse(context)?;
        let visibility = VisibilityKind::parse(context)?;
        let kind = match identifier!(context.cursor) {
            Some(keywords::Use) => {
                context.cursor.next();

                ItemKind::Use(UseNode::parse(context)?)
            }
            Some(keywords::Enum) => {
                context.cursor.next();

                ItemKind::Enum(EnumNode::parse(context)?)
            }
            Some(keywords::Fn) => {
                context.cursor.next();

                ItemKind::Function(FunctionNode::parse(context)?)
            }
            Some(keywords::Type) => {
                context.cursor.next();

                ItemKind::TypeAlias(TypeAliasNode::parse(context)?)
            }
            Some(keywords::Trait) => {
                context.cursor.next();

                ItemKind::Trait(TraitNode::parse(context)?)
            }
            Some(keywords::Const) => {
                context.cursor.next();

                ItemKind::Constant(ConstantNode::parse(context)?)
            }
            Some(keywords::Impl) => {
                context.cursor.next();

                ItemKind::Implement(ImplementNode::parse(context)?)
            }
            _ => {
                return if attributes.is_empty() {
                    Ok(None)
                } else {
                    Err(Error::Invalid)
                }
            }
        };

        Ok(Some(ItemNode {
            id: DUMMY_NODE_ID,
            attributes,
            visibility,
            kind,
        }))
    }
}
