use crate::{Error, Parse};
use danube_ast::{FunctionParameterNode, ImmutabilityKind, DUMMY_NODE_ID};

impl<'parse> Parse<'parse> {
    pub fn parse_function_parameter_nodes(
        &mut self,
    ) -> Result<(Option<ImmutabilityKind>, Vec<FunctionParameterNode>), Error> {
        if !symbol!(self.cursor => LeftParens) {
            return Err(Error::Invalid);
        }

        let immutability = if let Ok(immutability) = self.parse_immutability_kind() {
            if identifier!(self.cursor => SelfLower) {
                Some(immutability)
            } else if immutability == ImmutabilityKind::Nope {
                return Err(Error::Invalid);
            } else {
                None
            }
        } else if identifier!(self.cursor => SelfLower) {
            Some(ImmutabilityKind::Nope)
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

        Ok((immutability, parameters))
    }

    pub fn parse_function_parameter_node(&mut self) -> Result<FunctionParameterNode, Error> {
        let argument_label = self.parse_ident_node()?;
        let (parameter_label, ty) = if symbol!(self.cursor => Colon) {
            (None, self.parse_type_node()?)
        } else {
            (Some(self.parse_ident_node()?), self.parse_type_node()?)
        };

        Ok(FunctionParameterNode {
            id: DUMMY_NODE_ID,
            argument_label,
            parameter_label,
            ty,
        })
    }
}
