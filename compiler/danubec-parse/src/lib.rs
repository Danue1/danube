mod ast;
mod cursor;
mod error;

use cursor::Cursor;
use danubec_ast::Ast;
use danubec_token::Token;
use error::ParseError;

pub struct Parse;

impl Parse {
    pub fn parse(tokens: &[Token]) -> Result<Ast, ParseError> {
        use ast::Parse;

        Ast::parse(&mut Cursor::new(tokens))
    }
}
