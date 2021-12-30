use crate::Parse;
use danube_ast::{IdentNode, PathNode, VisibilityKind};
use danube_lex::Lex;
use danube_token::{Symbol, Token};

#[test]
fn current() {
    let source = "";
    let tokens: Vec<Token> = Lex::new(source).filter_map(|token| token.ok()).collect();

    assert_eq!(
        Parse::new(tokens.as_slice()).parse_visibility_kind(),
        Ok(VisibilityKind::Current)
    );
}

#[test]
fn public() {
    let source = "pub";
    let tokens: Vec<Token> = Lex::new(source).filter_map(|token| token.ok()).collect();

    assert_eq!(
        Parse::new(tokens.as_slice()).parse_visibility_kind(),
        Ok(VisibilityKind::Public)
    );
}

#[test]
fn restricted() {
    let source = "pub(foo)";
    let tokens: Vec<Token> = Lex::new(source).filter_map(|token| token.ok()).collect();

    assert_eq!(
        Parse::new(tokens.as_slice()).parse_visibility_kind(),
        Ok(VisibilityKind::Restricted(PathNode {
            idents: vec![IdentNode {
                symbol: Symbol::intern("foo")
            }]
        }))
    );
}
