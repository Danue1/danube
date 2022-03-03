use crate::{Context, Parse};
use danube_ast::{PathNode, VisibilityKind};
use danube_diagnostics::MessageBuilder;

impl Parse for VisibilityKind {
    type Output = VisibilityKind;

    fn parse(context: &mut Context) -> Result<VisibilityKind, ()> {
        if !identifier!(context.cursor => Pub) {
            return Ok(VisibilityKind::Current);
        }
        if !symbol!(context.cursor => LeftParens) {
            return Ok(VisibilityKind::Public);
        }

        let path = if let Some(path) = PathNode::parse(context)? {
            path
        } else {
            return context.report(MessageBuilder::error("Expected path").build());
        };
        let visibility_kind = VisibilityKind::Restricted(path);

        if symbol!(context.cursor => RightParens) {
            Ok(visibility_kind)
        } else {
            context.report(MessageBuilder::error("Expected `)`").build())
        }
    }
}
