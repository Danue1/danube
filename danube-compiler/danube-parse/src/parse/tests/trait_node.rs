use crate::Parse;
use danube_ast::{IdentNode, TraitNode};
use danube_lex::Lex;
use danube_token::{Symbol, Token};

#[test]
fn without_items() {
    let source = "Foo { }";
    let tokens: Vec<Token> = Lex::new(source).filter_map(|token| token.ok()).collect();

    assert_eq!(
        Parse::new(tokens.as_slice()).parse_trait_node(),
        Ok(TraitNode {
            ident: IdentNode {
                symbol: Symbol::intern("Foo"),
            },
            generics: vec![],
            inheritances: vec![],
            items: vec![],
        }),
    );
}
