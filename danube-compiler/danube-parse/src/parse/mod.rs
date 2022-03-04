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
mod named_struct_field;
mod package_node;
mod path_node;
mod pattern_node;
mod statement_node;
mod struct_field_kind;
mod struct_node;
mod trait_node;
mod type_alias_node;
mod type_kind;
mod type_node;
mod unnamed_struct_field;
mod use_node;
mod visibility_kind;

#[cfg(test)]
mod tests;

use crate::Context;
use danube_ast::PackageNode;
use danube_diagnostics::Diagnostics;
use danube_token::Token;
use std::cell::RefCell;

pub(crate) trait Parse {
    type Output;

    fn parse(context: &mut Context) -> Result<Self::Output, ()>;
}

#[allow(clippy::result_unit_err)]
pub fn parse<'parse>(
    token: &'parse [Token],
    diagnostics: &'parse RefCell<Diagnostics>,
) -> Result<PackageNode, ()> {
    let mut context = Context::new(token, diagnostics);

    PackageNode::parse(&mut context)
}
