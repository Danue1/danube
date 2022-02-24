use crate::Parse;
use danube_ast::IdentNode;
use danube_lex::Lex;
use danube_token::{Symbol, Token};

#[test]
fn ident_node() {
    let source = "hello";
    let tokens: Vec<Token> = Lex::new(source).filter_map(|token| token.ok()).collect();

    assert_eq!(
        Parse::new(tokens.as_slice()).parse_ident_node(),
        Ok(IdentNode {
            symbol: Symbol::intern("hello"),
        }),
    );
}
