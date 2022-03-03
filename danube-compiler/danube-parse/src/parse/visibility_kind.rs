use crate::{Context, Error, Parse};
use danube_ast::{PathNode, VisibilityKind};

impl Parse for VisibilityKind {
    type Output = VisibilityKind;

    fn parse(context: &mut Context) -> Result<VisibilityKind, Error> {
        if !identifier!(context.cursor => Pub) {
            return Ok(VisibilityKind::Current);
        }
        if !symbol!(context.cursor => LeftParens) {
            return Ok(VisibilityKind::Public);
        }

        let path = if let Some(path) = PathNode::parse(context)? {
            path
        } else {
            return Err(Error::Invalid);
        };
        let visibility_kind = VisibilityKind::Restricted(path);

        if symbol!(context.cursor => RightParens) {
            Ok(visibility_kind)
        } else {
            Err(Error::Invalid)
        }
    }
}
