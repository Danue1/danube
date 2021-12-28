use crate::{Error, Parse};
use danube_ast::{FunctionNode, FunctionParameterNode, TypeNode};

impl<'parse> Parse<'parse> {
    pub fn parse_function_node(&mut self) -> Result<FunctionNode, Error> {
        let ident = self.parse_ident_node()?;
        let generics = vec![];
        let self_type = None;
        let parameters = self.parse_function_parameter_nodes()?;
        let return_type = self.parse_function_return_type()?;
        let block = self.parse_block_node()?;

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

#[cfg(test)]
mod tests {
    use crate::Parse;
    use danube_ast::{FunctionNode, IdentNode};
    use danube_lex::Lex;
    use danube_token::{SymbolInterner, Token};

    #[test]
    #[ignore]
    fn empty_block() {
        let source = "hello();";
        let tokens: Vec<Token> = Lex::new(source).filter_map(|token| token.ok()).collect();
        let mut interner = SymbolInterner::default();

        assert_eq!(
            Parse::new(tokens.as_slice()).parse_function_node(),
            Ok(FunctionNode {
                ident: IdentNode {
                    symbol: interner.intern("hello"),
                },
                generics: vec![],
                self_type: None,
                parameters: vec![],
                return_type: None,
                block: None,
            })
        );
    }
}
