use crate::{Error, Parse};
use danube_ast::BlockNode;

impl<'parse> Parse<'parse> {
    pub fn parse_block_node(&mut self) -> Result<Option<BlockNode>, Error> {
        if !symbol!(self.cursor => LeftBrace) {
            return Ok(None);
        }

        let mut statements = vec![];

        while !symbol!(self.cursor => RightBrace) {
            let statement = self.parse_statement_node()?;
            statements.push(statement);
        }

        Ok(Some(BlockNode { statements }))
    }
}

#[cfg(test)]
mod tests {
    use crate::Parse;
    use danube_ast::BlockNode;
    use danube_lex::Lex;
    use danube_token::Token;

    #[test]
    fn emtpy() {
        let source = "";
        let tokens: Vec<Token> = Lex::new(source).filter_map(|token| token.ok()).collect();

        assert_eq!(Parse::new(tokens.as_slice()).parse_block_node(), Ok(None));
    }

    #[test]
    fn empty_block() {
        let source = "{ }";
        let tokens: Vec<Token> = Lex::new(source).filter_map(|token| token.ok()).collect();

        assert_eq!(
            Parse::new(tokens.as_slice()).parse_block_node(),
            Ok(Some(BlockNode { statements: vec![] }))
        );
    }
}
