use crate::Parse;
use danube_ast::IdentNode;

impl<'parse> Parse<'parse> {
    pub fn parse_ident_node(&mut self) -> Option<IdentNode> {
        match identifier!(self.cursor) {
            Some(identifier) => {
                let raw = identifier.value.to_string();

                self.cursor.next();

                Some(IdentNode { raw })
            }
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Parse;
    use danube_ast::IdentNode;
    use danube_lex::Lex;
    use danube_token::Token;

    #[test]
    fn ident_node() {
        let source = "hello";
        let tokens: Vec<Token> = Lex::new(source).filter_map(|token| token.ok()).collect();

        assert_eq!(
            Parse::new(tokens.as_slice()).parse_ident_node(),
            Some(IdentNode {
                raw: "hello".to_string()
            })
        );
    }
}
