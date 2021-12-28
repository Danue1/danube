mod attribute_node;
mod block_node;
mod constant_node;
mod enum_node;
mod expression_kind;
mod function_node;
mod ident_node;
mod implement_node;
mod item_node;
mod package_node;
mod path_node;
mod statement_node;
mod trait_node;
mod type_alias_node;
mod use_node;
mod visibility_kind;

use crate::{Cursor, Error, Resolver};
use danube_ast::PackageNode;
use danube_token::Token;

pub struct Parse<'parse> {
    cursor: Cursor<'parse>,
    resolver: Resolver,
}

impl<'parse> Parse<'parse> {
    pub fn new(tokens: &'parse [Token]) -> Self {
        Parse {
            cursor: Cursor::new(tokens),
            resolver: Resolver::new(),
        }
    }

    pub fn parse(&mut self) -> Result<PackageNode, Error> {
        self.parse_package_node()
    }
}

#[cfg(test)]
mod tests {
    use crate::Parse;
    use danube_ast::PackageNode;
    use danube_lex::Lex;
    use danube_token::Token;

    #[test]
    fn empty() {
        let source = "";
        let lexer = Lex::new(source);
        let tokens: Vec<Token> = lexer.filter_map(|token| token.ok()).collect();

        assert_eq!(
            Parse::new(tokens.as_slice()).parse(),
            Ok(PackageNode {
                attributes: vec![],
                items: vec![],
            })
        );
    }
}
