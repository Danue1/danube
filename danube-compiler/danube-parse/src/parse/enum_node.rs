use super::generic_node::GenericNodeList;
use crate::{Context, Parse};
use danube_ast::{EnumNode, EnumVariantNode, IdentNode};
use danube_diagnostics::MessageBuilder;
use danube_token::TokenKind;

impl Parse for EnumNode {
    type Output = EnumNode;

    fn parse(context: &mut Context) -> Result<Self::Output, ()> {
        let ident = IdentNode::parse(context)?;
        let generics = GenericNodeList::parse(context)?;

        match symbol!(context.cursor) {
            Some(TokenKind::Semicolon) => {
                context.cursor.next();

                Ok(EnumNode {
                    ident,
                    generics,
                    variants: vec![],
                })
            }
            Some(TokenKind::LeftBrace) => {
                context.cursor.next();

                let mut variants = vec![];

                while !symbol!(context.cursor => RightBrace) {
                    variants.push(EnumVariantNode::parse(context)?);

                    if !symbol!(context.cursor => Comma) && symbol!(context.cursor => RightBrace) {
                        break;
                    }
                }

                Ok(EnumNode {
                    ident,
                    generics,
                    variants,
                })
            }
            _ => context.report(MessageBuilder::error("Expected `{` or `;`").build()),
        }
    }
}
