use crate::{Context, Error, Parse};
use danube_ast::{PathNode, TypeKind};
use danube_token::TokenKind;

impl Parse for TypeKind {
    type Output = TypeKind;

    fn parse(context: &mut Context) -> Result<Self::Output, Error> {
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
                    Err(Error::Invalid)
                }
            }
            _ => {
                let path = if let Some(path) = PathNode::parse(context)? {
                    path
                } else {
                    return Err(Error::Invalid);
                };

                if !symbol!(context.cursor => LeftChevron) {
                    return Ok(TypeKind::Path(path));
                }

                let mut parameters = vec![TypeKind::parse(context)?];

                while symbol!(context.cursor => Comma) {
                    parameters.push(TypeKind::parse(context)?);
                }

                if symbol!(context.cursor => RightChevron) {
                    Ok(TypeKind::Generic(path, parameters))
                } else {
                    Err(Error::Invalid)
                }
            }
        }
    }
}
