use crate::{Context, Error, Parse};
use danube_ast::{EnumVariantKind, EnumVariantNode, IdentNode, TypeNode, DUMMY_NODE_ID};
use danube_token::TokenKind;

impl Parse for EnumVariantNode {
    type Output = EnumVariantNode;

    fn parse(context: &mut Context) -> Result<Self::Output, Error> {
        let ident = IdentNode::parse(context)?;

        match symbol!(context.cursor) {
            Some(TokenKind::LeftParens) => {
                context.cursor.next();

                let mut variants = vec![];

                while !symbol!(context.cursor => RightParens) {
                    variants.push(TypeNode::parse(context)?);

                    if !symbol!(context.cursor => Comma) && symbol!(context.cursor => RightParens) {
                        break;
                    }
                }

                Ok(EnumVariantNode {
                    id: DUMMY_NODE_ID,
                    ident,
                    kind: Some(EnumVariantKind::Unnamed(variants)),
                })
            }
            Some(TokenKind::LeftBrace) => {
                context.cursor.next();

                let mut variants = vec![];

                while !symbol!(context.cursor => RightBrace) {
                    let ident = IdentNode::parse(context)?;
                    let ty = if symbol!(context.cursor => Colon) {
                        TypeNode::parse(context)?
                    } else {
                        return Err(Error::Invalid);
                    };
                    variants.push((ident, ty));

                    if !symbol!(context.cursor => Comma) && symbol!(context.cursor => RightBrace) {
                        break;
                    }
                }

                Ok(EnumVariantNode {
                    id: DUMMY_NODE_ID,
                    ident,
                    kind: Some(EnumVariantKind::Named(variants)),
                })
            }
            _ => Ok(EnumVariantNode {
                id: DUMMY_NODE_ID,
                ident,
                kind: None,
            }),
        }
    }
}
