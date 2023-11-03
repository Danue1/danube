#[macro_use]
mod macros;
#[macro_use]
mod context;
mod parse;

use context::{Context, State};
use danubec_syntax_kind::SyntaxKind;
use rowan::GreenNode;

#[derive(Debug)]
pub struct Parse;

impl Parse {
    pub fn parse(mut tokens: Vec<(SyntaxKind, String)>) -> GreenNode {
        tokens.reverse();
        let mut context = Context::new(tokens);
        context.ast();
        context.builder.finish()
    }
}
