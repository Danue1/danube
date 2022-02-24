use crate::{Error, Parse};
use danube_ast::{FunctionNode, TypeNode};

impl<'parse> Parse<'parse> {
    pub fn parse_function_node(&mut self) -> Result<FunctionNode, Error> {
        let ident = self.parse_ident_node()?;
        let generics = self.parse_generic_nodes()?;
        let (self_type, parameters) = self.parse_function_parameter_nodes()?;
        let return_type = self.parse_function_return_type()?;
        let block = if symbol!(self.cursor => Semicolon) {
            None
        } else {
            Some(self.parse_block_node()?)
        };

        Ok(FunctionNode {
            ident,
            generics,
            self_type,
            parameters,
            return_type,
            block,
        })
    }

    fn parse_function_return_type(&mut self) -> Result<Option<TypeNode>, Error> {
        if symbol!(self.cursor => HyphenRightChevron) {
            Ok(Some(self.parse_type_node()?))
        } else {
            Ok(None)
        }
    }
}
