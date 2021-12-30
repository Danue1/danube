use crate::{Error, Parse};
use danube_ast::{FunctionNode, FunctionParameterNode, TypeNode};

impl<'parse> Parse<'parse> {
    pub fn parse_function_node(&mut self) -> Result<FunctionNode, Error> {
        let ident = self.parse_ident_node()?;
        let generics = vec![];
        let self_type = None;
        let parameters = self.parse_function_parameter_nodes()?;
        let return_type = self.parse_function_return_type()?;
        let block = if symbol!(self.cursor => LeftBrace) {
            Some(self.parse_block_node()?)
        } else {
            None
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

    fn parse_function_parameter_nodes(&mut self) -> Result<Vec<FunctionParameterNode>, Error> {
        std::todo!();
    }

    fn parse_function_return_type(&mut self) -> Result<Option<TypeNode>, Error> {
        std::todo!();
    }
}
