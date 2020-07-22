mod constant_node;
mod enum_node;
mod function_node;
mod ident_node;
mod struct_node;
mod trait_node;
mod type_alias_node;
mod type_node;
mod value_node;

use crate::*;
use constant_node::*;
use enum_node::enum_node;
use function_node::{function_node, trait_item_function_node};
use ident_node::ident_node;
use nom::{
  branch::alt,
  combinator::{all_consuming, map},
  multi::many0,
  sequence::tuple,
};
use struct_node::struct_node;
use trait_node::trait_node;
use type_alias_node::type_alias_node;
use type_node::type_node;
use value_node::value_usize;

pub fn parse(source: &str) -> std::result::Result<ModuleNode, Error> {
  match all_consuming(map(
    tuple((
      many0(map(
        tuple((ignore_token0, positioned(item_node))),
        |(_, item)| item,
      )),
      ignore_token0,
    )),
    |(item_list, _)| item_list,
  ))(nom_locate::LocatedSpan::new(source))
  {
    Ok((_, item_list)) => Ok(ModuleNode {
      ident: None,
      item_list,
    }),
    Err(nom::Err::Error(error)) => Err(error),
    _ => std::unreachable!(),
  }
}

fn item_node(s: Span) -> Result<ItemNode> {
  alt((
    map(struct_node, ItemNode::Struct),
    map(enum_node, ItemNode::Enum),
    map(function_node, ItemNode::Function),
    map(type_alias_node, ItemNode::TypeAlias),
    map(trait_node, ItemNode::TraitNode),
    map(constant_node, ItemNode::Constant),
  ))(s)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test() {
    let path_list = [
      // "tests/const",
      // "tests/enum",
      // "tests/function",
      "tests/struct",
      // "tests/trait",
    ];

    for path in path_list.iter() {
      for entry in std::fs::read_dir(path).unwrap() {
        if let Ok(entry) = entry {
          let path = entry.path();
          let source = std::fs::read_to_string(&path).unwrap();
          if let Err(error) = parse(source.as_str()) {
            dbg!(path);
            dbg!(error);
            panic!();
          };
        }
      }
    }
  }
}
