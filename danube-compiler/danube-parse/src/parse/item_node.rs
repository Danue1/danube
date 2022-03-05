use super::attribute_node::ItemAttributeNodeList;
use crate::{Context, Parse};
use danube_ast::{
    ConstantNode, EnumNode, FunctionNode, ImplementNode, ItemKind, ItemNode, ModNode, StructNode,
    TraitNode, TypeAliasNode, UseNode, VisibilityKind,
};
use danube_diagnostics::MessageBuilder;
use danube_token::keywords;

pub(crate) struct ItemNodeList;

impl Parse for ItemNodeList {
    type Output = Vec<ItemNode>;

    fn parse(context: &mut Context) -> Result<Self::Output, ()> {
        let mut items = vec![];

        while let Some(item) = ItemNode::parse(context)? {
            items.push(item);
        }

        Ok(items)
    }
}

impl Parse for ItemNode {
    type Output = Option<ItemNode>;

    fn parse(context: &mut Context) -> Result<Self::Output, ()> {
        let attributes = ItemAttributeNodeList::parse(context)?;
        let visibility = VisibilityKind::parse(context)?;
        let kind = match identifier!(context.cursor) {
            Some(keywords::Mod) => {
                context.cursor.next();

                ItemKind::Mod(ModNode::parse(context)?)
            }
            Some(keywords::Use) => {
                context.cursor.next();

                ItemKind::Use(UseNode::parse(context)?)
            }
            Some(keywords::Enum) => {
                context.cursor.next();

                ItemKind::Enum(EnumNode::parse(context)?)
            }
            Some(keywords::Struct) => {
                context.cursor.next();

                ItemKind::Struct(StructNode::parse(context)?)
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
                    context.report(
                        MessageBuilder::error(
                            "Expected `use`, `enum`, `fn`, `type`, `trait`, `const`, or `impl`",
                        )
                        .build(),
                    )
                }
            }
        };

        Ok(Some(ItemNode {
            attributes,
            visibility,
            kind,
        }))
    }
}
