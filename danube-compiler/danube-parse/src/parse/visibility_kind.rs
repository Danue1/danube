use crate::{Error, Parse};
use danube_ast::VisibilityKind;

impl<'parse> Parse<'parse> {
    pub fn parse_visibility_kind(&mut self) -> Result<VisibilityKind, Error> {
        if !identifier!(self.cursor => Pub) {
            return Ok(VisibilityKind::Current);
        }
        if !symbol!(self.cursor => LeftParens) {
            return Ok(VisibilityKind::Public);
        }

        let path = if let Some(path) = self.parse_path_node()? {
            path
        } else {
            return Err(Error::Invalid);
        };
        let visibility_kind = VisibilityKind::Restricted(path);

        if symbol!(self.cursor => RightParens) {
            Ok(visibility_kind)
        } else {
            Err(Error::Invalid)
        }
    }
}
