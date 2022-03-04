use crate::{Context, Parse};
use danube_ast::{GenericTypeNode, PathNode, TypeKind};
use danube_diagnostics::MessageBuilder;
use danube_token::TokenKind;

impl Parse for TypeKind {
    type Output = TypeKind;

    fn parse(context: &mut Context) -> Result<Self::Output, ()> {
        match context.cursor.peek().kind {
            TokenKind::LeftParens => {
                context.cursor.next();

                let mut parameters = vec![TypeKind::parse(context)?];

                while symbol!(context.cursor => Comma) {
                    parameters.push(TypeKind::parse(context)?);
                }

                if symbol!(context.cursor => RightParens) {
                    Ok(TypeKind::Tuple(parameters))
                } else {
                    context.report(MessageBuilder::error("Expected `)`").build())
                }
            }
            _ => {
                let path = if let Some(path) = PathNode::parse(context)? {
                    path
                } else {
                    return context.report(MessageBuilder::error("Expected type path").build());
                };

                if !symbol!(context.cursor => LeftChevron) {
                    return Ok(TypeKind::Path(path));
                }

                let mut parameters = vec![TypeKind::parse(context)?];

                while symbol!(context.cursor => Comma) {
                    parameters.push(TypeKind::parse(context)?);
                }

                if symbol!(context.cursor => RightChevron) {
                    Ok(TypeKind::Generic(GenericTypeNode { path, parameters }))
                } else {
                    context.report(MessageBuilder::error("Expected `>`").build())
                }
            }
        }
    }
}
