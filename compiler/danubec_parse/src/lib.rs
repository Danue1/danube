#![warn(clippy::all)]
#![allow(unused)]

#[macro_use]
pub mod context;

#[macro_use]
mod tokens;

mod node;
mod pratt;

pub use context::*;

use pratt::*;
use tokens::*;

use danubec_ast::Root;
use danubec_lex::Lex;
use danubec_syntax::{AstNode, SyntaxNode};

pub fn parse(source: &str) -> Root {
    let mut context = Context::new();
    let mut lex = Lex::new(source);
    context.root(&mut lex);

    Root::cast(context.finish()).expect("ICE: root not found")
}
