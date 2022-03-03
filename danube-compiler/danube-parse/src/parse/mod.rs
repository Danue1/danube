mod argument_node;
mod attribute_node;
mod block_node;
mod constant_node;
mod enum_node;
mod enum_variant_node;
mod expression_kind;
mod expression_node;
mod function_node;
mod function_parameter_node;
mod generic_node;
mod ident_node;
mod immutability_kind;
mod implement_item_node;
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

use crate::{Cursor, Error};
use danube_ast::PackageNode;
use danube_token::Token;

pub(crate) struct Context<'context> {
    pub(crate) cursor: Cursor<'context>,
}

impl<'context> Context<'context> {
    pub(crate) fn new(token: &'context [Token]) -> Self {
        Context {
            cursor: Cursor::new(token),
        }
    }
}

pub(crate) trait Parse {
    type Output;

    fn parse(context: &mut Context) -> Result<Self::Output, Error>;
}

pub fn parse(token: &[Token]) -> Result<PackageNode, Error> {
    let mut context = Context::new(token);

    PackageNode::parse(&mut context)
}
