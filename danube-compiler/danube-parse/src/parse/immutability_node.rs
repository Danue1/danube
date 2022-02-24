use crate::{Error, Parse};
use danube_ast::ImmutabilityKind;

impl<'parse> Parse<'parse> {
    pub fn parse_immutability_kind(&mut self) -> Result<ImmutabilityKind, Error> {
        if identifier!(self.cursor => Mut) {
            Ok(ImmutabilityKind::Nope)
        } else {
            Ok(ImmutabilityKind::Yes)
        }
    }
}
