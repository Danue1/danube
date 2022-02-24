use crate::{Error, Parse};
use danube_ast::ImmutablityKind;

impl<'parse> Parse<'parse> {
  pub fn parse_immutablity_kind(&mut self) -> Result<ImmutablityKind, Error> {
    if identifier!(self.cursor => Mut) {
      Ok(ImmutablityKind::Nope)
    } else {
      Ok(ImmutablityKind::Yes)
    }
  }
}
