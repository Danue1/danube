mod parse_identifier;
mod parse_ignorable;
mod parse_illegal;
mod parse_numeric;
mod parse_string;
mod parse_symbol;

use crate::*;
use nom::{
  branch::*, bytes::complete::*, character::complete::*, combinator::*, multi::*, sequence::*,
};
use parse_identifier::{Identifier, parse_identifier};
use parse_ignorable::parse_ignorable;
use parse_illegal::parse_illegal;
use parse_numeric::{Numeric, parse_numeric};
use parse_string::parse_string;
use parse_symbol::parse_symbol;
pub use token::*;

type LexSpan<'a> = nom_locate::LocatedSpan<&'a str>;
type LexResult<'a, T> = nom::IResult<LexSpan<'a>, T, LexError<'a>>;

pub fn lex(s: &str) -> LexResult<Vec<Token>> {
    fold_many0(
        alt((map(parse_ignorable, |_| None), map(parse_token, Some))),
        vec![],
        |mut token_list, token| {
            if let Some(token) = token {
                token_list.push(token);
            }
            token_list
        },
    )(nom_locate::LocatedSpan::new(s))
}

fn parse_token(s: LexSpan) -> LexResult<Token> {
    alt((
        map(parse_string, Token::StringLiteral),
        map(parse_numeric, |numeric| match numeric {
            Numeric::Int(n) => Token::IntLiteral(n),
            Numeric::Float(n) => Token::FloatLiteral(n),
        }),
        map(parse_identifier, |identifier| match identifier {
            Identifier::Unreserved(i) => Token::Identifier(i),
            Identifier::Reserved(i) => Token::Keyword(i),
        }),
        map(parse_symbol, Token::Symbol),
        map(parse_illegal, |_| Token::Illegal),
    ))(s)
}
