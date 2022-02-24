use crate::{Error, Parse};
use danube_ast::TypeKind;
use danube_token::TokenKind;

impl<'parse> Parse<'parse> {
    pub fn parse_type_kind(&mut self) -> Result<TypeKind, Error> {
        match self.cursor.peek().kind {
            TokenKind::LeftParens => {
                self.cursor.next();

                let mut parameters = vec![self.parse_type_kind()?];

                while symbol!(self.cursor => Comma) {
                    parameters.push(self.parse_type_kind()?);
                }

                if symbol!(self.cursor => RightParens) {
                    Ok(TypeKind::Tuple(parameters))
                } else {
                    Err(Error::Invalid)
                }
            }
            _ => {
                let path = if let Some(path) = self.parse_path_node()? {
                    path
                } else {
                    return Err(Error::Invalid);
                };

                if !symbol!(self.cursor => LeftChevron) {
                    return Ok(TypeKind::Path(path));
                }

                let mut parameters = vec![self.parse_type_kind()?];

                while symbol!(self.cursor => Comma) {
                    parameters.push(self.parse_type_kind()?);
                }

                if symbol!(self.cursor => RightChevron) {
                    Ok(TypeKind::Generic(path, parameters))
                } else {
                    Err(Error::Invalid)
                }
            }
        }
    }
}
