mod attribute_node;
mod block_node;
mod compound_assign_kind;
mod compound_assign_node;
mod condition_node;
mod conditional_node;
mod constant_node;
mod enum_node;
mod enum_variant_node;
mod expression_kind;
mod field_node;
mod for_node;
mod function_argument_node;
mod function_node;
mod generic_node;
mod ident_node;
mod identifier;
mod immutablity_kind;
mod implement_item_kind;
mod implement_node;
mod implement_trait_node;
mod infix_operator_kind;
mod item_kind;
mod item_node;
mod keyword;
mod let_node;
mod literal_value_kind;
mod loop_node;
mod module_node;
mod named_struct_node;
mod path_node;
mod pattern_kind;
mod pattern_match_node;
mod program_node;
mod return_node;
mod statement_kind;
mod static_node;
mod struct_fields_kind;
mod struct_named_fields_node;
mod struct_node;
mod struct_unnamed_fields_node;
mod symbol;
mod trait_item_constant_node;
mod trait_item_function_node;
mod trait_item_kind;
mod trait_node;
mod type_alias_node;
mod type_array_node;
mod type_kind;
mod unary_operator_kind;
mod unnamed_struct_node;
mod use_extra_kind;
mod use_kind;
mod use_node;
mod use_root_ident_kind;
mod use_root_node;
mod visibility_kind;
mod while_node;

use crate::*;
use attribute_node::parse_attribute_node;
use block_node::parse_block_node;
use compound_assign_kind::parse_compound_assign_kind;
use compound_assign_node::parse_compound_assign_node;
use condition_node::parse_condition_node;
use conditional_node::parse_conditional_node;
use constant_node::parse_constant_node;
use enum_node::parse_enum_node;
use enum_variant_node::parse_enum_variant_node;
use expression_kind::parse_expression_kind;
use field_node::parse_field_node;
use for_node::parse_for_node;
use function_argument_node::parse_function_argument_node;
use function_node::parse_function_node;
use generic_node::parse_generic_node;
use ident_node::parse_ident_node;
use identifier::parse_identifier;
use immutablity_kind::parse_immutablity_kind;
use implement_item_kind::parse_implement_item_kind;
use implement_node::parse_implement_node;
use implement_trait_node::parse_implement_trait_node;
use infix_operator_kind::parse_infix_operator_kind;
use item_kind::parse_item_kind;
use item_node::parse_item_node;
use keyword::parse_keyword;
use let_node::parse_let_node;
use literal_value_kind::{parse_int, parse_literal_value_kind};
use loop_node::parse_loop_node;
use module_node::parse_module_node;
use named_struct_node::parse_named_struct_node;
use nom::{branch::*, bytes::complete::*, combinator::*, multi::*, sequence::*};
use path_node::parse_path_node;
use pattern_kind::parse_pattern_kind;
use pattern_match_node::parse_pattern_match_node;
use program_node::parse_program_node;
use return_node::parse_return_node;
use statement_kind::parse_statement_kind;
use static_node::parse_static_node;
use struct_fields_kind::parse_struct_fields_kind;
use struct_named_fields_node::parse_struct_named_fields_node;
use struct_node::parse_struct_node;
use struct_unnamed_fields_node::parse_struct_unnamed_fields_node;
use symbol::parse_symbol;
use trait_item_constant_node::parse_trait_item_constant_node;
use trait_item_function_node::parse_trait_item_function_node;
use trait_item_kind::parse_trait_item_kind;
use trait_node::parse_trait_node;
use type_alias_node::parse_type_alias_node;
use type_array_node::parse_type_array_node;
use type_kind::parse_type_kind;
use unary_operator_kind::parse_unary_operator_kind;
use unnamed_struct_node::parse_unnamed_struct_node;
use use_extra_kind::parse_use_extra_kind;
use use_kind::parse_use_kind;
use use_node::parse_use_node;
use use_root_ident_kind::parse_use_root_ident_kind;
use use_root_node::parse_use_root_node;
use visibility_kind::parse_visibility_kind;
use while_node::parse_while_node;

type ParseResult<'a, T> = nom::IResult<Tokens<'a>, T, Error<'a>>;

pub fn parse(s: Tokens) -> ParseResult<ProgramNode> {
  parse_program_node(s)
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
