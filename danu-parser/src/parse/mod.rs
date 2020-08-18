mod assign_sugar_kind;
mod assign_sugar_node;
mod binary_operator_kind;
mod constant_node;
mod enum_node;
mod enum_variant_node;
mod expression_conditional_node;
mod expression_node;
mod field_node;
mod function_argument_node;
mod function_node;
mod generic_node;
mod ident_node;
mod identifier;
mod implement_item_node;
mod implement_node;
mod implement_trait_node;
mod keyword;
mod let_mut_node;
mod let_node;
mod literal_value_node;
mod loop_node;
mod named_struct_node;
mod path_node;
mod pattern_match_node;
mod pattern_node;
mod return_node;
mod statement_conditional_node;
mod statement_node;
mod static_node;
mod struct_fields_node;
mod struct_named_fields_node;
mod struct_node;
mod struct_unnamed_fields_node;
mod symbol;
mod trait_item_constant_node;
mod trait_item_function_node;
mod trait_item_node;
mod trait_node;
mod type_alias_node;
mod type_array_node;
mod type_node;
mod unary_operator_kind;
mod unnamed_struct_node;
mod use_extra;
mod use_kind;
mod use_node;
mod use_root_ident;
mod use_root_node;
mod while_node;

use crate::*;
use assign_sugar_kind::parse_assign_sugar_kind;
use assign_sugar_node::parse_assign_sugar_node;
use binary_operator_kind::parse_binary_operator_kind;
use constant_node::parse_constant_node;
use enum_node::parse_enum_node;
use enum_variant_node::parse_enum_variant_node;
use expression_conditional_node::parse_expression_conditional_node;
use expression_node::parse_expression_node;
use field_node::parse_field_node;
use function_argument_node::parse_function_argument_node;
use function_node::parse_function_node;
use generic_node::parse_generic_node;
use ident_node::parse_ident_node;
use identifier::parse_identifier;
use implement_item_node::parse_implement_item_node;
use implement_node::parse_implement_node;
use implement_trait_node::parse_implement_trait_node;
use keyword::parse_keyword;
use let_mut_node::parse_let_mut_node;
use let_node::parse_let_node;
use literal_value_node::{parse_int, parse_literal_value_node};
use loop_node::parse_loop_node;
use named_struct_node::parse_named_struct_node;
use nom::{branch::*, bytes::complete::*, combinator::*, multi::*, sequence::*};
use path_node::parse_path_node;
use pattern_match_node::parse_pattern_match_node;
use pattern_node::parse_pattern_node;
use return_node::parse_return_node;
use statement_conditional_node::parse_statement_conditional_node;
use statement_node::parse_statement_node;
use static_node::parse_static_node;
use struct_fields_node::parse_struct_fields_node;
use struct_named_fields_node::parse_struct_named_fields_node;
use struct_node::parse_struct_node;
use struct_unnamed_fields_node::parse_struct_unnamed_fields_node;
use symbol::parse_symbol;
use trait_item_constant_node::parse_trait_item_constant_node;
use trait_item_function_node::parse_trait_item_function_node;
use trait_item_node::parse_trait_item_node;
use trait_node::parse_trait_node;
use type_alias_node::parse_type_alias_node;
use type_array_node::parse_type_array_node;
use type_node::parse_type_node;
use unary_operator_kind::parse_unary_operator_kind;
use unnamed_struct_node::parse_unnamed_struct_node;
use use_extra::parse_use_extra;
use use_kind::parse_use_kind;
use use_node::parse_use_node;
use use_root_ident::parse_use_root_ident;
use use_root_node::parse_use_root_node;
use while_node::parse_while_node;

type ParseResult<'a, T> = nom::IResult<Tokens<'a>, T, Error<'a>>;

pub fn parse(s: Tokens) -> ParseResult<ModuleNode> {
  all_consuming(map(many0(parse_item), |item_list| ModuleNode {
    ident: None,
    item_list,
  }))(s)
}

fn parse_item(s: Tokens) -> ParseResult<ItemNode> {
  alt((
    map(parse_use_node, ItemNode::Use),
    map(parse_struct_node, ItemNode::Struct),
    map(parse_enum_node, ItemNode::Enum),
    map(parse_function_node, ItemNode::Function),
    map(parse_type_alias_node, ItemNode::TypeAlias),
    map(parse_trait_node, ItemNode::Trait),
    map(parse_constant_node, ItemNode::Constant),
    map(parse_static_node, ItemNode::Static),
    map(parse_implement_node, ItemNode::Implement),
    map(parse_implement_trait_node, ItemNode::ImplementTrait),
  ))(s)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn testcases() {
    let path_list = [
      "tests/const",
      "tests/control_flow",
      "tests/enum",
      "tests/expression",
      "tests/function",
      "tests/impl",
      "tests/static",
      "tests/struct",
      "tests/trait",
      "tests/type_alias",
      "tests/use",
      "tests/value",
    ];

    for path in path_list.iter() {
      for entry in std::fs::read_dir(path).unwrap() {
        if let Ok(entry) = entry {
          let path = entry.path();
          let source = std::fs::read_to_string(&path).unwrap();
          let (_, token_list) = lex(source.as_str()).unwrap();
          dbg!(path, token_list.len());
          parse(Tokens::new(&token_list)).unwrap();
        }
      }
    }
  }
}
