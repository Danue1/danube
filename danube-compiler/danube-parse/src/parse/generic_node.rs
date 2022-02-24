use crate::{Error, Parse};
use danube_ast::GenericNode;

impl<'parse> Parse<'parse> {
  pub fn parse_generic_nodes(&mut self) -> Result<Vec<GenericNode>, Error> {
    if !symbol!(self.cursor => LeftChevron) {
      return Ok(vec![]);
    }

    let mut generics = vec![];

    while !symbol!(self.cursor => RightChevron) {
      generics.push(self.parse_generic_node()?);

      if !symbol!(self.cursor => Comma) {
        if symbol!(self.cursor => RightChevron) {
          break;
        }

        return Err(Error::Invalid);
      }
    }

    Ok(generics)
  }

  pub fn parse_generic_node(&mut self) -> Result<GenericNode, Error> {
    let ident = self.parse_ident_node()?;
    let traits = if symbol!(self.cursor => Colon) {
      let mut paths = if let Some(path) = self.parse_path_node()? {
        vec![path]
      } else {
        return Err(Error::Invalid);
      };

      while symbol!(self.cursor => Plus) {
        if let Some(path) = self.parse_path_node()? {
          paths.push(path);
        } else {
          return Err(Error::Invalid);
        }
      }

      paths
    } else {
      vec![]
    };
    let default = if symbol!(self.cursor => Eq) {
      if let Some(path) = self.parse_path_node()? {
        Some(path)
      } else {
        return Err(Error::Invalid);
      }
    } else {
      None
    };

    Ok(GenericNode {
      ident,
      traits,
      default,
    })
  }
}
