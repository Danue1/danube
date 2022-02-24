mod attribute_node;
mod block_node;
mod constant_node;
mod enum_node;
mod enum_variant_node;
mod expression_kind;
mod function_node;
mod function_parameter_node;
mod generic_node;
mod ident_node;
mod immutablity_node;
mod implement_node;
mod item_node;
mod package_node;
mod path_node;
mod pattern_node;
mod statement_node;
mod trait_node;
mod type_alias_node;
mod type_kind;
mod type_node;
mod use_node;
mod visibility_kind;

#[cfg(test)]
mod tests;

use crate::{Cursor, Error, Resolver};
use danube_ast::PackageNode;
use danube_token::Token;

pub struct Parse<'parse> {
    cursor: Cursor<'parse>,
    resolver: Resolver,
}

impl<'parse> Parse<'parse> {
    pub fn new(tokens: &'parse [Token]) -> Self {
        Parse {
            cursor: Cursor::new(tokens),
            resolver: Resolver::new(),
        }
    }

    pub fn parse(&mut self) -> Result<PackageNode, Error> {
        self.parse_package_node()
    }
}
