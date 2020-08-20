#![warn(clippy::all)]

mod ast;
mod error;
mod execute;
mod lex;
mod parse;

pub use ast::*;
pub use error::*;
pub use execute::*;
pub use lex::*;
pub use parse::*;

pub fn compile(s: &str) {
  let (_, token_list) = lex(s).unwrap();
  let (_, module) = parse(Tokens::new(&token_list)).unwrap();
  execute(module)
}
