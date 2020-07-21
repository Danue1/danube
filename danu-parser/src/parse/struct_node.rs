use super::*;
use crate::*;
use nom::{
  branch::alt,
  bytes::complete::tag,
  combinator::{map, opt},
  multi::separated_list,
  sequence::tuple,
};

pub(super) fn struct_node(s: Span) -> Result<StructNode> {
  map(
    tuple((
      tag("struct"),
      ignore_token1,
      positioned(ident_node),
      ignore_token0,
      struct_fields_node,
    )),
    |(_, _, ident, _, fields)| StructNode { ident, fields },
  )(s)
}

fn struct_fields_node(s: Span) -> Result<StructFieldsNode> {
  alt((
    map(struct_unnamed_fields_node, StructFieldsNode::Unnamed),
    map(struct_named_fields_node, StructFieldsNode::Named),
  ))(s)
}

fn struct_unnamed_fields_node(s: Span) -> Result<StructUnnamedFieldsNode> {
  map(
    tuple((
      left_parens,
      ignore_token0,
      unnamed_field_list,
      ignore_token0,
      opt(tuple((comma, ignore_token0))),
      right_parens,
      ignore_token0,
      semicolon,
    )),
    |(_, _, node_list, _, _, _, _, _)| StructUnnamedFieldsNode { node_list },
  )(s)
}

fn struct_named_fields_node(s: Span) -> Result<StructNamedFieldsNode> {
  map(
    tuple((
      left_brace,
      ignore_token0,
      named_field_list,
      ignore_token0,
      opt(tuple((comma, ignore_token0))),
      right_brace,
    )),
    |(_, _, node_list, _, _, _)| StructNamedFieldsNode { node_list },
  )(s)
}

fn unnamed_field_list(s: Span) -> Result<Vec<Positioned<TypeNode>>> {
  separated_list(
    tuple((ignore_token0, comma, ignore_token0)),
    positioned(unnamed_field),
  )(s)
}

fn named_field_list(s: Span) -> Result<Vec<(Positioned<IdentNode>, Positioned<TypeNode>)>> {
  separated_list(tuple((ignore_token0, comma, ignore_token0)), named_field)(s)
}

fn unnamed_field(s: Span) -> Result<TypeNode> {
  type_node(s)
}

fn named_field(s: Span) -> Result<(Positioned<IdentNode>, Positioned<TypeNode>)> {
  map(
    tuple((
      positioned(ident_node),
      ignore_token0,
      colon,
      ignore_token0,
      positioned(type_node),
    )),
    |(ident_node, _, _, _, type_node)| (ident_node, type_node),
  )(s)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test() {
    let source = r#"struct Foo { bar: [[Bar; 4]; 4] }"#;
    dbg!(parse(source));

    let source = r#"struct Foo(Bar);"#;
    dbg!(parse(source));
  }
}
