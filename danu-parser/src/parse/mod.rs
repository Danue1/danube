mod struct_node;
mod value_node;

use crate::*;
use nom::{
  combinator::{all_consuming, map},
  multi::many0,
  sequence::tuple,
};
use struct_node::struct_node;
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
  map(struct_node, ItemNode::Struct)(s)
}
