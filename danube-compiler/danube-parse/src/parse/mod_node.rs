use super::attribute_node::ItemAttributeNodeList;
use crate::{Context, Parse};
use danube_ast::{IdentNode, ItemNode, ModNode};
use danube_diagnostics::MessageBuilder;
use danube_token::TokenKind;

impl Parse for ModNode {
    type Output = ModNode;

    fn parse(context: &mut Context) -> Result<Self::Output, ()> {
        let ident = IdentNode::parse(context)?;
        let attributes = ItemAttributeNodeList::parse(context)?;
        match context.cursor.peek().kind {
            TokenKind::Semicolon => {
                context.cursor.next();

                Ok(ModNode {
                    attributes,
                    ident,
                    items: vec![],
                })
            }
            TokenKind::LeftBrace => {
                context.cursor.next();

                let mut items = vec![];

                while symbol!(context.cursor => RightBrace) {
                    if let Some(item) = ItemNode::parse(context)? {
                        items.push(item);
                    } else if symbol!(context.cursor => RightBrace) {
                        break;
                    } else {
                        return context.report(
                            MessageBuilder::error("Expected item or '}' after '{'").build(),
                        );
                    }

                    if !symbol!(context.cursor => Comma) && symbol!(context.cursor => RightBrace) {
                        break;
                    }
                }

                Ok(ModNode {
                    attributes,
                    ident,
                    items,
                })
            }
            _ => {
                let kind = context.cursor.peek().kind;

                context.report(
                    MessageBuilder::error(&format!(
                        "Expected `;` or `{{` after `mod` but found `{:#?}`",
                        kind,
                    ))
                    .build(),
                )
            }
        }
    }
}
