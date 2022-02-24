use crate::Parse;
use danube_ast::{FunctionNode, IdentNode};
use danube_lex::Lex;
use danube_token::{Symbol, Token};

#[test]
#[ignore]
fn empty_block() {
    let source = "hello();";
    let tokens: Vec<Token> = Lex::new(source).filter_map(|token| token.ok()).collect();

    assert_eq!(
        Parse::new(tokens.as_slice()).parse_function_node(),
        Ok(FunctionNode {
            ident: IdentNode {
                symbol: Symbol::intern("hello"),
            },
            generics: vec![],
            self_type: None,
            parameters: vec![],
            return_type: None,
            block: None,
        }),
    );
}
