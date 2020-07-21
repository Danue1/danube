mod enum_node;
mod ident_node;
mod struct_node;
mod type_node;
mod value_node;

use crate::*;
use enum_node::enum_node;
use ident_node::ident_node;
use nom::{
  branch::alt,
  combinator::{all_consuming, map},
  multi::many0,
  sequence::tuple,
};
use struct_node::struct_node;
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
  ))(s)
}
