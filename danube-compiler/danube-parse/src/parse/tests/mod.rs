mod attribute_node;
mod block_node;
mod constant_node;
mod expression_kind;
mod function_node;
mod ident_node;
mod package_node;
mod path_node;
mod pattern_node;
mod statement_node;
mod type_kind;
mod use_node;
mod visibility_kind;

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
