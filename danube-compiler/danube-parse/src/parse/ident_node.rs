use crate::{Error, Parse};
use danube_ast::IdentNode;

impl<'parse> Parse<'parse> {
    pub fn parse_ident_node(&mut self) -> Result<IdentNode, Error> {
        match identifier!(self.cursor) {
            Some(&symbol) => {
                self.cursor.next();

                Ok(IdentNode { symbol })
            }
            None => Err(Error::Invalid),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Parse;
    use danube_ast::IdentNode;
    use danube_lex::Lex;
    use danube_token::{SymbolInterner, Token};

    #[test]
    fn ident_node() {
        let source = "hello";
        let tokens: Vec<Token> = Lex::new(source).filter_map(|token| token.ok()).collect();
        let mut interner = SymbolInterner::default();

        assert_eq!(
            Parse::new(tokens.as_slice()).parse_ident_node(),
            Ok(IdentNode {
                symbol: interner.intern("hello")
            })
        );
    }
}
