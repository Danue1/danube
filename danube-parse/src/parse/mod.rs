mod parse_attribute_node;
mod parse_attributed;
mod parse_block_node;
mod parse_expression_kind;
mod parse_float;
mod parse_function_node;
mod parse_function_parameter_node;
mod parse_function_parameter_node_list;
mod parse_ident_node;
mod parse_identifier;
mod parse_int;
mod parse_item_kind;
mod parse_keyword;
mod parse_let_node;
mod parse_literal_kind;
mod parse_module;
mod parse_module_node;
mod parse_path_node;
mod parse_pattern_kind;
mod parse_statement_kind;
mod parse_string;
mod parse_symbol;
mod parse_type_kind;

use crate::Error;
use danube_ast::*;
use danube_lex::*;
use nom::{branch::*, bytes::complete::*, combinator::*, multi::*, sequence::*};
use parse_attribute_node::parse_attribute_node;
use parse_attributed::parse_attributed;
use parse_block_node::parse_block_node;
use parse_expression_kind::parse_expression_kind;
use parse_float::parse_float;
use parse_function_node::parse_function_node;
use parse_function_parameter_node::parse_function_parameter_node;
use parse_function_parameter_node_list::parse_function_parameter_node_list;
use parse_ident_node::parse_ident_node;
use parse_identifier::parse_identifier;
use parse_int::parse_int;
use parse_item_kind::parse_item_kind;
use parse_keyword::parse_keyword;
use parse_let_node::parse_let_node;
use parse_literal_kind::parse_literal_kind;
use parse_module::parse_module;
use parse_module_node::parse_module_node;
use parse_path_node::parse_path_node;
use parse_pattern_kind::parse_pattern_kind;
use parse_statement_kind::parse_statement_kind;
use parse_string::parse_string;
use parse_symbol::parse_symbol;
use parse_type_kind::parse_type_kind;

type ParseResult<'a, T> = nom::IResult<Tokens<'a>, T, Error>;

pub fn from_tokens(t: Tokens) -> Result<Module, Error> {
    into(parse_module(t))
}

pub fn from_str(s: &str) -> Result<Module, Error> {
    into(parse_module(Tokens::new(&lex(s)?)))
}

fn into(result: ParseResult<Module>) -> Result<Module, Error> {
    match result {
        Ok((_, program)) => Ok(program),
        Err(error) => Err(error.into()),
    }
}

#[cfg(test)]
fn parse_by<T, F>(s: &str, f: F) -> T
where
    F: FnOnce(Tokens) -> ParseResult<T>,
{
    match f(Tokens::new(&lex(s).unwrap())) {
        Ok((_, node)) => node,
        Err(error) => {
            dbg!(error);
            panic!();
        }
    }
}
