use crate::Parse;
use danube_ast::{IdentNode, TraitNode, DUMMY_NODE_ID};
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
                id: DUMMY_NODE_ID,
                symbol: Symbol::intern("Foo"),
            },
            generics: vec![],
            inheritances: vec![],
            items: vec![],
        }),
    );
}
