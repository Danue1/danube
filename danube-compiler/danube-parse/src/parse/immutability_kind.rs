use crate::{Context, Error, Parse};
use danube_ast::ImmutabilityKind;

impl Parse for ImmutabilityKind {
    type Output = ImmutabilityKind;

    fn parse(context: &mut Context) -> Result<Self::Output, Error> {
        if identifier!(context.cursor => Mut) {
            Ok(ImmutabilityKind::Nope)
        } else {
            Ok(ImmutabilityKind::Yes)
        }
    }
}
