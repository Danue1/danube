use crate::{Context, Parse};
use danube_ast::ImmutabilityKind;

impl Parse for ImmutabilityKind {
    type Output = ImmutabilityKind;

    fn parse(context: &mut Context) -> Result<Self::Output, ()> {
        if identifier!(context.cursor => Mut) {
            Ok(ImmutabilityKind::Nope)
        } else {
            Ok(ImmutabilityKind::Yes)
        }
    }
}
