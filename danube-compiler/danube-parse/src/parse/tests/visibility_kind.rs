use crate::{Context, Parse};
use danube_ast::{IdentNode, PathNode, VisibilityKind, DUMMY_NODE_ID};
use danube_lex::Lex;
use danube_token::{Symbol, Token};

#[test]
fn current() {
    let source = "";
    let tokens: Vec<Token> = Lex::new(source).filter_map(|token| token.ok()).collect();

    assert_eq!(
        VisibilityKind::parse(&mut Context::new(tokens.as_slice())),
        Ok(VisibilityKind::Current),
    );
}

#[test]
fn public() {
    let source = "pub";
    let tokens: Vec<Token> = Lex::new(source).filter_map(|token| token.ok()).collect();

    assert_eq!(
        VisibilityKind::parse(&mut Context::new(tokens.as_slice())),
        Ok(VisibilityKind::Public),
    );
}

#[test]
fn restricted() {
    let source = "pub(foo)";
    let tokens: Vec<Token> = Lex::new(source).filter_map(|token| token.ok()).collect();

    assert_eq!(
        VisibilityKind::parse(&mut Context::new(tokens.as_slice())),
        Ok(VisibilityKind::Restricted(PathNode {
            segments: vec![IdentNode {
                id: DUMMY_NODE_ID,
                symbol: Symbol::intern("foo"),
            }],
        })),
    );
}
