use crate::{Error, Parse};
use danube_ast::{FunctionParameterNode, ImmutablityKind};

impl<'parse> Parse<'parse> {
  pub fn parse_function_parameter_nodes(
    &mut self,
  ) -> Result<(Option<ImmutablityKind>, Vec<FunctionParameterNode>), Error> {
    if !symbol!(self.cursor => LeftParens) {
      return Err(Error::Invalid);
    }

    let immutablity = if let Ok(immutablity) = self.parse_immutablity_kind() {
      if identifier!(self.cursor => SelfLower) {
        Some(immutablity)
      } else if immutablity == ImmutablityKind::Nope {
        return Err(Error::Invalid);
      } else {
        None
      }
    } else if identifier!(self.cursor => SelfLower) {
      Some(ImmutablityKind::Nope)
    } else {
      None
    };

    let mut parameters = vec![];

    while !symbol!(self.cursor => RightParens) {
      parameters.push(self.parse_function_parameter_node()?);

      if !symbol!(self.cursor => Comma) {
        if symbol!(self.cursor => RightParens) {
          break;
        }

        return Err(Error::Invalid);
      }
    }

    Ok((immutablity, parameters))
  }

  pub fn parse_function_parameter_node(&mut self) -> Result<FunctionParameterNode, Error> {
    let argument_label = self.parse_ident_node()?;
    let (parameter_label, ty) = if symbol!(self.cursor => Colon) {
      (None, self.parse_type_node()?)
    } else {
      (Some(self.parse_ident_node()?), self.parse_type_node()?)
    };

    Ok(FunctionParameterNode {
      argument_label,
      parameter_label,
      ty,
    })
  }
}
